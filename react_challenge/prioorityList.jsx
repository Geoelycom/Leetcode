// Challenge: Priority List
// State: [items, setItems] = useState([])—array of { id, text, priority }.
// Actions:
// “Add Item” (id: add-button)—adds { id: items.length + 1, text: "Item {id}", priority: 0 }.
// “Up Priority” (id: up-${id})—increments priority (no max).
// Display: <Item>—prop item.

import React, { useState } from 'react';

function PriorityList() {
  const [items, setItems] = useState([]);

  const addItem = () => {
    setItems((prevItem) => [
      ...prevItem,
      {
        id: prevItem.length + 1,
        text: `Item ${prevItem.length + 1}`,
        priority: 0
      }
    ])
  }

  const upPriority = (id) => {
     setItems((prevItem) => prevItem.map((item) => {
      if (id === item.id) {
        return {
          ...item,
          priority: item.priority + 1
        }
      }
      return item;
     }))
  }  

  return (
    <div id="PriorityList" className="container">
      <div className="row">
        <button id="add-button" onClick={addItem}>Add Item</button>
      </div>
      <div className="row" id="items">
        {/* Render items */}
        {items.map((item) => (
          <Item key={item.id} item={item}  onUp={() => upPriority(item.id)}/>
        ))}
      </div>
    </div>
  );
}

function Item({ item, onUp }) {
  return (
    <div>
      <span>{item.text} (Priority: {item.priority})</span>
      <button id={`up-${item.id}`} onClick={onUp}>Up</button>
    </div>
  );
}

export default PriorityList;