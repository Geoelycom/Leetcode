import { useState, useEffect } from 'react';
import React from 'react';
// import products from './product'
import './index.css'

function App() {
  const [products, setProduct] = useState([]);
  const [searchItem, setSearchItem] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchproducts = async () => {
      setLoading(true)
      try {
        const response = await fetch('https://fakestoreapi.com/products')
        if (!response.ok) {
          throw new Error('Failed to fetch products')
        }
        const data = await response.json()
        setProduct(data)
        console.log(data)
      } catch (error) {
        setError(error.message)
      } finally {
        setLoading(false)
      }
    };
    fetchproducts()
  }, [])

   const handleSearch = (e) => {
    setSearchItem(e.target.value);
   }

  // create a filtered list of products
  const filteredProducts = products.filter((item) => item.title.toLowerCase().includes(searchItem.toLowerCase()))

   const handleDelete = (id) => {
    setProduct((prevProducts) => prevProducts.filter((item) => item.id !== id ))
   }

  return (
    <div className="product_list">
      <div className="input">
      <input type="text" 
      className="input_text"
      placeholder="search for products"
      value={searchItem}
      onChange={handleSearch}
      >
      </input>
      </div>
      <h2>Product List</h2>
      {loading ? (
        <p>Loading...</p>
      ) : error ? (<p style={{color: 'red'}}>{error}</p>) : filteredProducts.length === 0 ? (
        <p> No product available</p>
      ): (
        <ul className="product_list_item"> 
        {filteredProducts.map((item) => (
          <li key={item.id}>
            <p>Name: {item.title}</p>
            <p>Price: {item.price}</p>
            <p>Category: {item.category}</p>
            <button className="btn btn-danger" onClick={() => handleDelete(item.id)}>
             Delete item
         </button>
          </li>
        ))}
      </ul>
      )}
    </div>
  )
}

export default App
