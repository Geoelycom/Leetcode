// Challenge: User Fetcher
// Goal: Fetch users from a mock API, display them, and add new ones locally.
// API: Use https://jsonplaceholder.typicode.com/users (returns [{ id, name, ... }, ...]).
// State: [users, setUsers] = useState([]).
// Actions:
// Fetch on mount—loads initial users.
// “Add User” (id: add-button)—adds { id: users.length + 1, name: "User {id}" }.
// Display: <User>—prop user.


import React, { useState, useEffect } from 'react';

function UserFetcher() {
  const [users, setUsers] = useState([]);



useEffect(() => {
const fetchUser = async () => {
    try {
      const response = await fetch(`https://jsonplaceholder.typicode.com/users`);
      if (!response.ok){
        throw new Error('couldnt fetch data')
      }
      const data =  await response.json();
      setUsers(data);
    } catch(error) {
      console.error("Error fetching users:", error);
    }
}
fetchUser();
}, [])



const addUser = () => {
  setUsers((prevUsers) => [
    ...prevUsers,
    {
      id: prevUsers.length + 1,
      name: `User ${prevUsers.length + 1}`,
    },
  ]);
};

  return (
    <div id="UserFetcher" className="container">
      <div className="row">
        <button id="add-button" onClick={addUser}>Add User</button>
      </div>
      <div className="row" id="users">
        {/* Render users */}
        {users.map((user) => (
          <User key={user.id} user={user} />
        ))}
      </div>
    </div>
  );
}




function User({ user }) {
  return <div>{user.name}</div>;
}

export default UserFetcher;