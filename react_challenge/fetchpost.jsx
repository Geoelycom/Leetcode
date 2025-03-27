// Goal: Fetch users from a mock API, display them, and add new ones locally.
// API: Use https://jsonplaceholder.typicode.com/users (returns [{ id, name, ... }, ...]).
// State: [users, setUsers] = useState([]).
// Display: <User>—prop user.


// Limiting the search to posts with an even ID only.
// Making the search case-insensitive (already done in toLowerCase()).
// Showing a message when no posts match the search.
// Add two new states:
// Update fetchPost() to:
// Set loading to true before fetching.
// Catch errors and update error state.
// Set loading to false after fetching.

import React, { useState, useEffect } from 'react';
const PostList = () => {
  const [post, setPost] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(false);
  const [searchItem, setSearchItem] = useState("");


useEffect(() => {
  const fetchPost = async () => {
    setLoading(true);
    setError(false);
    try {
      const response = await fetch("https://jsonplaceholder.typicode.com/posts");
      if (!resonse.ok){
        throw new Error("Error fetching posts");
      }
      const data = await response.json();
      // filter post based on even
      setPost(data.filter((post) => post.id % 2 === 0));
    } catch(error) {
      setError(" could not fetch data form api:", error);
    } finally {
      setLoading(false);
    }
  };
  fetchPost();
  }, []);

  // Filter Post based on searchItem
  const filteredPosts = post.filter((post) => {
   return post.title.toLowerCase().includes(searchItem.toLowerCase());
  })
 
  const handleSearchItem = (e) => {
    setSearchItem(e.target.value)
  }

  return (
    <div>
    <h2>Posts</h2>
    {/* Search Input */}
    <input
      type="text"
      placeholder="Search posts..."
      value={searchIterm}
      onChange={handleSearchItem}
    />
    {filteredPosts.length === 0 && <p>No post match search item</p>}
    {/* Display Filtered Posts */}

     {loading ? (
      <p>Loading posts...</p>
    ) : error ? (
      <p>{error}</p>
    ) : (
      // Display Posts
      <ul>
        {filteredPosts.map((post) => (
          <li key={post.id}>{post.title}</li>
        ))}
      </ul>
    )}

  </div>
);
};

export default PostList;


