# LightRAG: Knowledge Graph Merging with LLMs

## Overview

LightRAG is a system for building and maintaining a knowledge graph from unstructured documents using Large Language Models (LLMs). It extracts entities and relations from document chunks, merges descriptions, and stores both the knowledge graph and vector embeddings for efficient retrieval and reasoning.

## Why LLMs?

This project leverages LLMs for two main tasks:
- **Summarization:** When entity or relation descriptions become too long after merging, an LLM is called to summarize the combined text into a concise, informative description.
- **Embedding:** LLMs (or embedding models) are used to generate vector representations of entities and relations for efficient retrieval and semantic search.

## Merging Logic

When merging an extracted entity (or relation) with an existing one, the new description is concatenated with the existing description (separated by a separator). To prevent descriptions from growing too long, LightRAG uses an LLM to summarize the combined text into a single, cohesive description once it exceeds a certain length.

This is implemented in the functions:

- `merge_nodes_and_edges` (in `operate.py`)
- `_merge_nodes_then_upsert`
- `_merge_edges_then_upsert`

Let's break down what happens within these functions. The merge_nodes_and_edges function, as called in Stage 2 of process_document from interview_0.md, takes all_entities and all_relations extracted from the document chunks and passes them to _merge_nodes_then_upsert and _merge_edges_then_upsert respectively.

### Example

If the entity "John" appears in multiple document chunks, each chunk may yield a different description. These are merged by concatenation (and possibly summarization) into a single entity description. Similarly, if "John" and "Wine" are related in multiple chunks, their relation descriptions are merged in the same way.

## Pseudocode

```python
async def merge_nodes_and_edges(all_entities, all_relations, knowledge_graph, entity_vector_database, relation_vector_database):
    # Group entities and relations by name
    entities_grouped_by_name = group_entities_by_name(all_entities)
    relations_grouped_by_entity_name_pairs = group_relations_by_entity_name_pairs(all_relations)

    # Merge entities and relations
    with global_asyncio_lock:
        for entity_name, entities in entities_grouped_by_name.items():
            await _merge_nodes_then_upsert(entity_name, entities, knowledge_graph, entity_vector_database)
        for (entity1_name, entity2_name), relations in relations_grouped_by_entity_name_pairs.items():
            await _merge_edges_then_upsert(entity1_name, entity2_name, relations, knowledge_graph, relation_vector_database)

async def _merge_nodes_then_upsert(entity_name, entities, knowledge_graph, entity_vector_database):
    # Concatenate descriptions
    if entity_name in knowledge_graph:
        entities += [await knowledge_graph.get(entity_name)]  # I/O: Database Read
    merged_description = concatenate_descriptions(entities)

    # Summarize if too long
    if len(merged_description) > MAX_DESCRIPTION_LENGTH:
        merged_description = await CALL_LLM_For_Summarization(merged_description)  # I/O: LLM Call

    # Embed and store
    new_entity = <construct new entity with merged_description>
    entity_embedding = await CALL_LLM_For_Embedding(new_entity)
    await knowledge_graph.StoreOrMergeEntity(new_entity)
    await entity_vector_database.StoreEmbedding(new_entity.id, entity_embedding)

async def _merge_edges_then_upsert(entity1_name, entity2_name, relations, knowledge_graph, relation_vector_database):
    # Concatenate descriptions
    if (entity1_name, entity2_name) in knowledge_graph:
        relations += [await knowledge_graph.get((entity1_name, entity2_name))]
    merged_description = concatenate_descriptions(relations)

    # Summarize if too long
    if len(merged_description) > MAX_DESCRIPTION_LENGTH:
        merged_description = await CALL_LLM_For_Summarization(merged_description)

    # Embed and store
    new_relation = <construct new relation with merged_description>
    relation_embedding = await CALL_LLM_For_Embedding(new_relation)
    await knowledge_graph.StoreOrMergeRelation(new_relation)
    await relation_vector_database.StoreEmbedding(new_relation.id, relation_embedding)
```

## Bottleneck: Concurrency and LLM Calls

> **Note:**  
> The `merge_nodes_and_edges` function uses a global lock (`with global_asyncio_lock:`) to synchronize merging. This means that even if you process multiple documents in parallel, only one can merge entities or relations at a time. This creates a bottleneck and prevents true parallelism, especially when LLM calls (which are slow) are involved.
>
> **Possible Solution:**  
> Consider using more granular locks (e.g., per-entity or per-relation) or lock-free concurrency patterns to allow multiple merges to proceed in parallel, improving throughput.

## License

MIT License

## Acknowledgements

- Inspired by modern RAG and knowledge graph research.
- Uses LLMs for summarization and embedding.