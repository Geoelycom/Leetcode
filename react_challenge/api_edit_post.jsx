import { useState, useEffect } from "react";

const API_URL = "https://jsonplaceholder.typicode.com/posts";

const PostEditorAPI = () => {
  const [posts, setPosts] = useState([]);
  const [editingPost, setEditingPost] = useState(null);
  const [editText, setEditText] = useState("");

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(API_URL);
        if (!response.ok) {
          throw new Error('Error fetching data');
        }
        const data = await response.json();
      setPosts(data.slice(0, 5));
    } catch (error) {
      console.error('couldnt load data:', error);
    }
  };
  fetchData();
}, []);


  const handleEdit = (post) => {
    setEditingPost(post.id);
    setEditText(post.title);
  };

  const handleEditEvent = (e) => {
    setEditText(e.target.value)
  }

  const handleSave = async (id) => {
    const updatedPost = { title: editText };

    // Simulating an API update
    await fetch(`${API_URL}/${id}`, {
      method: "PUT",
      body: JSON.stringify(updatedPost),
      headers: { "Content-Type": "application/json" },
    });

    setPosts(posts.map(post => post.id === id ? { ...post, title: editText } : post));
    setEditingPost(null);
  };


  return (
    <div>
      <h2>Posts (From API)</h2>
      {posts.map((post) => (
        <div key={post.id}>
          {editingPost === post.id ? (
            <>
              <input
                type="text"
                value={editText}
                onChange={handleEditEvent}
              />
              <button onClick={() => handleSave(post.id)}>Save</button>
            </>
          ) : (
            <>
              <h3>{post.title}</h3>
              <button onClick={() => handleEdit(post)}>Edit</button>
            </>
          )}
        </div>
      ))}
    </div>
  );
};

export default PostEditorAPI;
