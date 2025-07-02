import asyncio
import hashlib
import uuid
from collections import defaultdict
from dataclasses import dataclass
from typing import List, Optional

# Constants
MAX_DESCRIPTION_LENGTH = 1000
DESCRIPTION_SEPARATOR = " | "

# Data classes for entities and relations
@dataclass
class Entity:
    id: str
    name: str
    description: str

@dataclass
class Relation:
    id: str
    entity1_name: str
    entity2_name: str
    description: str

# Helper functions
def generate_entity_id(entity_name: str) -> str:
    """Generate a unique ID for an entity based on its name."""
    return f"entity_{hashlib.md5(entity_name.encode()).hexdigest()[:8]}"

def generate_relation_id(entity1_name: str, entity2_name: str) -> str:
    """Generate a unique ID for a relation based on entity names."""
    relation_key = f"{entity1_name}_{entity2_name}"
    return f"relation_{hashlib.md5(relation_key.encode()).hexdigest()[:8]}"

def concatenate_descriptions(items: List) -> str:
    """Concatenate descriptions from a list of entities or relations."""
    descriptions = []
    for item in items:
        if hasattr(item, 'description') and item.description:
            descriptions.append(item.description.strip())
    return DESCRIPTION_SEPARATOR.join(descriptions)

# Placeholder functions for LLM calls (to be implemented)
async def CALL_LLM_For_Summarization(text: str) -> str:
    """Placeholder for LLM summarization call."""
    # TODO: Implement actual LLM call for summarization
    return text[:MAX_DESCRIPTION_LENGTH]  # Simple truncation as fallback

async def CALL_LLM_For_Embedding(item) -> List[float]:
    """Placeholder for LLM embedding call."""
    # TODO: Implement actual LLM call for embedding generation
    return [0.0] * 768  # Placeholder embedding vector

# Helper function to group entities by name
def group_entities_by_name(entities):
    grouped = defaultdict(list)
    for entity in entities:
        grouped[entity.name].append(entity)
    return grouped

# Helper function to group relations by (entity1_name, entity2_name)
def group_relations_by_entity_name_pairs(relations):
    grouped = defaultdict(list)
    for relation in relations:
        key = (relation.entity1_name, relation.entity2_name)
        grouped[key].append(relation)
    return grouped

# Global lock dictionaries (or use a LockManager class)
entity_locks = defaultdict(asyncio.Lock)  # Maps entity_name to asyncio.Lock
relation_locks = defaultdict(asyncio.Lock)  # Maps (entity1_name, entity2_name) to asyncio.Lock

async def merge_nodes_and_edges(all_entities, all_relations, knowledge_graph, entity_vector_database, relation_vector_database):
    # Group entities and relations by name
    entities_grouped_by_name = group_entities_by_name(all_entities)
    relations_grouped_by_entity_name_pairs = group_relations_by_entity_name_pairs(all_relations)

    # Merge entities and relations with fine-grained locks
    tasks = []
    for entity_name, entities in entities_grouped_by_name.items():
        tasks.append(_merge_nodes_then_upsert(entity_name, entities, knowledge_graph, entity_vector_database, entity_locks[entity_name]))
    for (entity1_name, entity2_name), relations in relations_grouped_by_entity_name_pairs.items():
        relation_key = (entity1_name, entity2_name)
        tasks.append(_merge_edges_then_upsert(entity1_name, entity2_name, relations, knowledge_graph, relation_vector_database, relation_locks[relation_key]))

    # Run all tasks concurrently
    await asyncio.gather(*tasks)

async def _merge_nodes_then_upsert(entity_name, entities, knowledge_graph, entity_vector_database, entity_lock):
    async with entity_lock:
        if entity_name in knowledge_graph:
            entities += [await knowledge_graph.get(entity_name)]
        merged_description = concatenate_descriptions(entities)
        if len(merged_description) > MAX_DESCRIPTION_LENGTH:
            merged_description = await CALL_LLM_For_Summarization(merged_description)
        new_entity = Entity(
            id=generate_entity_id(entity_name),
            name=entity_name,
            description=merged_description
        )
        entity_embedding = await CALL_LLM_For_Embedding(new_entity)
        await knowledge_graph.StoreOrMergeEntity(new_entity)
        await entity_vector_database.StoreEmbedding(new_entity.id, entity_embedding)

async def _merge_edges_then_upsert(entity1_name, entity2_name, relations, knowledge_graph, relation_vector_database, relation_lock):
    async with relation_lock:
        if (entity1_name, entity2_name) in knowledge_graph:
            relations += [await knowledge_graph.get((entity1_name, entity2_name))]
        merged_description = concatenate_descriptions(relations)
        if len(merged_description) > MAX_DESCRIPTION_LENGTH:
            merged_description = await CALL_LLM_For_Summarization(merged_description)
        new_relation = Relation(
            id=generate_relation_id(entity1_name, entity2_name),
            entity1_name=entity1_name,
            entity2_name=entity2_name,
            description=merged_description
        )
        relation_embedding = await CALL_LLM_For_Embedding(new_relation)
        await knowledge_graph.StoreOrMergeRelation(new_relation)
        await relation_vector_database.StoreEmbedding(new_relation.id, relation_embedding)
