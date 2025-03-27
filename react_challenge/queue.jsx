import React, { useState } from 'react';

function UserQueue() {
  const [queue, setQueue] = useState([]);


  const addUser = () => {
    setQueue((prevQueue) => [
      ...prevQueue,
      {
        id: prevQueue.length + 1,
        name: `User ${prevQueue.length + 1}`,
      }
    ])
  }

  const serveNext = () => {
    setQueue((prevQueue) => prevQueue.slice(1));
  }

  return (
    <div id="UserQueue" className="container">
      <div className="row">
        <button id="add-button" onClick={addUser}>Add User</button>
        <button id="serve-button" onClick={serveNext}>Serve Next</button>
      </div>
      <div className="row" id="queue">
       {queue.map((user) => (
        <User key={user.id} user={user} />
       ))}
      </div>
    </div>
  );
}

function User({ user }) {
  return <div>{user.name}</div>;
}

export default UserQueue;