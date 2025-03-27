import React from "react";
import { useState } from "react";

function TaskList() {
  //  we assume that we have an array of objects that our map loops over, so instead of using the three cards div as provided by the html blob we only use one and allow it to map over the array and display all the data in the array.
  const [task, setTask] = useState([]);
  return (
    <div class="task-list">
      {task.map((tasks) => (
      <div className="task" key={tasks.id}>
        <h3 className="task-title">{tasks.title}</h3>
        <p className="task-status">{tasks.status}</p>
      </div>
      ))}
    </div>
  );
}

export default TaskList;