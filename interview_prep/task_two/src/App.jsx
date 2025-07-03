import { useState } from 'react';

function App({ fruits }) {
  const [data, updateData] = useState(fruits);
  const [name, setName] = useState('');
  const [color, setColor] = useState('');

  /**
   * Add a new fruit to the list of fruits.
   * @function
   */
  const handleAdd = () => {
    const newFruit = {
      id: Date.now(), // simple unique ID
      name,
      color
    };
    updateData(prev => [...prev, newFruit]);
    setName('');
    setColor('');
  };

  /**
   * Deletes a fruit from the list by its unique ID.
   * @param {number} id - The unique identifier of the fruit to be deleted.
   */

  const handleDelete = (id) => {
    updateData(prev => prev.filter(fruit => fruit.id !== id));
  };

  return (
    <div>
      <h1>Fruit List</h1>

      <ul>
        {data.map(fruit => (
          <li key={fruit.id}>
            <h2>Name: {fruit.name}</h2>
            <p>Color: {fruit.color}</p>
            <button onClick={() => handleDelete(fruit.id)}>Delete</button>
          </li>
        ))}
      </ul>

      <div>
        <h3>Add New Fruit</h3>
        <input
          type="text"
          placeholder="Name"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <input
          type="text"
          placeholder="Color"
          value={color}
          onChange={e => setColor(e.target.value)}
        />
        <button onClick={handleAdd}>Add</button>
      </div>
    </div>
  );
}

export default App;