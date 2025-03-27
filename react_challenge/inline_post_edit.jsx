import { useState } from "react";

const PostEditor = () => {
  const [posts, setPosts] = useState([
    { id: 1, title: "First Post" },
    { id: 2, title: "Second Post" }
  ]);

  const [editingId, setEditingId] = useState(null); // Track which post is being edited
  const [editText, setEditText] = useState(""); // Store new text while editing

  const handleEdit = (id, currentTitle) => {
    setEditingId(id); // Set the editing state
    setEditText(currentTitle); // Fill the input with the current title
  };

  const handleSave = (id) => {
    setPosts(posts.map(post => (post.id === id ? { ...post, title: editText } : post)));
    setEditingId(null); // Exit editing mode
  };

  return (
    <div>
      <h2>Editable Posts</h2>
      {posts.map((post) => (
        <div key={post.id}>
          {editingId === post.id ? (
            <>
              <input 
                type="text" 
                value={editText} 
                onChange={(e) => setEditText(e.target.value)} 
              />
              <button onClick={() => handleSave(post.id)}>Save</button>
            </>
          ) : (
            <>
              <p>{post.title}</p>
              <button onClick={() => handleEdit(post.id, post.title)}>Edit</button>
            </>
          )}
        </div>
      ))}
    </div>
  );
};

export default PostEditor;


// updating nested object in states

const [user, setUser] = useState({
  name: 'Ada',
  profile: {
    bio: 'Frontend dev',
    socials: {
      twitter: '@ada_dev',
      github: 'adadev'
    }
  }
});

// the object has name, profile, socials

setUser({
  ...user, // spread top level (user)
  profile: {
    ...user.profile, // spread second level (profile)
    socials: {
      ...user.profile.socials, // spread third level (socials)
      github: 'ada-dev-2025' // only update this one key
    }
  }
});

// we make use of Immer if the states are too nested and difficult to work with
const [person, updatePerson] = useImmer({
  name: 'Niki de Saint Phalle',
  artwork: {
    title: 'Blue Nana',
    city: 'Hamburg',
    image: 'https://i.imgur.com/Sd1AgUOm.jpg',
  }
});


updatePerson(draft => {
  draft.artwork.title = 'bayern',
});