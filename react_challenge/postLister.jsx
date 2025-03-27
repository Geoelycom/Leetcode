// Challenge: Post Lister
// State: [posts, setPosts] = useState([])—{ id, title }.
// API: https://jsonplaceholder.typicode.com/posts—fetch on mount.
// Actions:
// “Clear” (id: clear-button)—resets to [].
// Display: <Post>—prop post.



import React, { useState, useEffect } from 'react';

function PostLister() {
  const [posts, setPosts] = useState([]);
        // post would be an array of objects = [{id: 1, title: "title1"}, {id: 2, title: "title2"}]
        // setPost would be a function to update the post state
      useEffect(() => {
        const fetchpost = async () => {
          try {
            const response = await fetch("https://jsonplaceholder.typicode.com/posts")
            if(!response.ok){
              throw new Error('fetching of post failed')
            }
            const data = await response.json();
            setPosts(data);
          } catch(error) {
            console.error("couldn't fetch post:", error);
          }
          // const response = await fetch("https://jsonplaceholder.typicode.com/posts");
          // const data = await response.json();
          // setPosts(data);
        }
        fetchpost();
      }, []);

      // deleting the whole rendering item from the ui
      const ClearPost = () => {
        setPosts([]);
      }

  return (
    <div id="PostLister" className="container">
      <div className="row">
        <button id="clear-button" onClick={ClearPost}>Clear</button>
      </div>
      <div className="row" id="posts">
      {posts.map((post) => {
        return <Post key={post.id} post={post} />
      })}
      </div>
    </div>
  );
}

function Post({ post }) {
  return <div>{post.title}</div>;
}

export default PostLister;