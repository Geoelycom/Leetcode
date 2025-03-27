// Overview
// Build a TaskList component to manage a list of tasks. Users can add tasks, mark them as complete, and clear completed tasks—all with array state.

// Requirements
// State: [tasks, setTasks] = useState([])—array of task objects:
// Each task: { id: number, text: string, completed: boolean }.
// Start empty.
// Display:
// Render tasks via <Task> child component.
// Prop: task—pass each task object.
// Buttons:
// “Add Task” (id: add-button)—adds a new task (text: “Task {id}”, e.g., “Task 1”).
// Each task: “Toggle” (id: toggle-{id})—toggles completed.
// “Clear Completed” (id: clear-button)—removes completed tasks.


import React, { useState } from 'react';

function TaskList() {
  const [tasks, setTasks] = useState([]);

  const addTask = () => {
    // { id: number, text: string, completed: boolean } = Task
    setTasks((prevTasks) => [
      ...prevTasks,
      {
        id: prevTasks.length + 1,
        text: `Task ${prevTasks.length + 1}`,
        completed: false
      }
    ])

  }

  const ToggleTask = (id) => {
    setTasks((prevTasks) => prevTasks.map((task) => {
        if (id === task.id) {
          return {
            ...task,
            completed: !task.completed
          };
        }
        return task;
      })
    );
  }

  const ClearCompleted = () => {
    setTasks((prevTasks) => prevTasks.filter((task) => !task.completed));
  }

    // Your code here

  return (
    <div id="TaskList" className="container">
      <div className="row">
        <button id="add-button" onClick={addTask} className="btn btn-primary">Add Task</button>
        <button id="clear-button" onClick={ClearCompleted}>Clear Completed</button>
      </div>
      <div className="row" id="tasks">
        {tasks.map((task) => (
          <Task key={task.id} task={task} onToggle={() => ToggleTask(task.id)} />
        ))}
        {/* Render tasks */}
      </div>
    </div>
  );
}

function Task({ task, onToggle }) {
  return (
    <div>
      <span>{task.text} - {task.completed ? 'Done' : 'Pending'}</span>
      <button id={`toggle-${task.id}`} onClick={onToggle}>Toggle</button>
    </div>
  );
}

export default TaskList;



const additem = () => {
  setItems(...items, 'orange')
}

setItems(items.filter(item => item !== 'apple'))

setUsers(users.map(user) => user.id === 2 ? { ...user, name: 'Bobby'}: user)
 const updateUser = () => {
  setUsers(users.map(user => {
  if (user.id === 2) {
    return {
      ...user, 
      name: 'Bobby'
    }
  }
  return user;
}))}

const addUser = () => {
setUsers([
  ...users,
  {
    id: 3,
    name: 'charlie'
  }
])
}

const addItem = () => {
  setItems((prevItem) => [
    ...prevItem,
    {
      id: prevItem.length + 1,
      text: `Item ${prevItem.length + 1}`,
      priority: 0
    }
  ])
}




setUser({
  ...user,
  role: "Admin"
})