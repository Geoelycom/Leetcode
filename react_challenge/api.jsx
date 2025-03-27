import React, { useEffect, useState } from "react";

function PostsList() {
  const [posts, setPosts] = useState([]); // State to store posts
  const [loading, setLoading] = useState(true); // Loading state
  const [error, setError] = useState(null); // Error state

  useEffect(() => {
    fetch("https://jsonplaceholder.typicode.com/posts")
      .then((response) => response.json()) // Convert response to JSON
      .then((data) => {
        setPosts(data); // Store data in state
        setLoading(false); // Stop loading
      })
      .catch((err) => {
        setError(err.message); // Handle errors
        setLoading(false);
      });
  }, []);

}


import { useState, useEffect } from "react";

function Posts() {
  const [posts, setPosts] = useState([]); // State to store posts

  useEffect(() => {
    const fetchPosts = async () => {
      try {
        const response = await fetch("https://jsonplaceholder.typicode.com/posts");
        const data = await response.json();
        setPosts(data.filter((post) => post.id % 2 === 0)); // fetch only post with even id
      } catch (error) {
        console.error("Error fetching posts:", error);
      }
    };

    fetchPosts(); // Call the async function
  }, []); // Runs only once when the component mounts

  return (
    <div>
      <h2>First 10 Posts</h2>
      <ul>
        {posts.map((post) => (
          <li key={post.id}>{post.title}</li>
        ))}
      </ul>
    </div>
  );
}

export default Posts;




// components/Product.js
const Product = ({ name, price, category }) => {
  return (
    <div>
      <h3>{name}</h3>
      <p>Price: ${price}</p>
      <p>Category: {category}</p>
      <button>Buy now</button>
    </div>
  );
};

export default Product;


// components/ProductList.js
import Product from "./Product";

const ProductList = () => {
  const products = [
    { id: 1, name: "MacBook Pro", price: 2500, category: "newsales" },
    { id: 2, name: "iPhone 15", price: 1200, category: "discounted_goods" },
    { id: 3, name: "Apple Watch", price: 500, category: "flash_sales"},
  ];

  return (
    <div>
      <h2>Product List</h2>
      {products.map((item) => (
        <Product key={item.id} name={item.name} price={item.price} category={item.category} />
      ))}
    </div>
  );
};

export default ProductList;

