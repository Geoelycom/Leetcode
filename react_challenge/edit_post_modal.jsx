import { useState } from "react";

const PostManager = () => {
  const [posts, setPosts] = useState([
    { id: 1, title: "Post One", content: "This is the first post" },
    { id: 2, title: "Post Two", content: "This is the second post" },
  ]);

  const [selectedPost, setSelectedPost] = useState(null);
  const [editData, setEditData] = useState({ title: "", content: "" });

  const handleEdit = (post) => {
    setSelectedPost(post);
    setEditData({ title: post.title, content: post.content });
  };

  const handleSave = () => {
    setPosts(
      posts.map((post) =>
        post.id === selectedPost.id ? { ...post, ...editData } : post
      )
    );
    setSelectedPost(null);
  };

  return (
    <div>
      <h2>Posts</h2>
      {posts.map((post) => (
        <div key={post.id}>
          <h3>{post.title}</h3>
          <p>{post.content}</p>
          <button onClick={() => handleEdit(post)}>Edit</button>
        </div>
      ))}

      {selectedPost && (
        <div className="modal">
          <h2>Edit Post</h2>
          <input
            type="text"
            value={editData.title}
            onChange={(e) => setEditData({ ...editData, title: e.target.value })}
          />
          <textarea
            value={editData.content}
            onChange={(e) =>
              setEditData({ ...editData, content: e.target.value })
            }
          />
          <button onClick={handleSave}>Save</button>
        </div>
      )}
    </div>
  );
};

export default PostManager;

