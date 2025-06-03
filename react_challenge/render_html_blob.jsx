import React from 'react';
import { useState } from 'react';

tasks = [{ id: 1, name: "Finish report" }, { id: 2, name: "Call client" }];
function  RenderBlob() {
const [data, setData] = useState(tasks)

return (
  <div class="task-list bg-gray-100 p-4 rounded">
  <h2 class="text-2xl font-bold mb-4">Tasks</h2>
  <div class="tasks">
    {data.map((task) => (
         <div class="task bg-white p-3 mb-2 rounded shadow" key={task.id}>
         <span class="task-name">{task.name}</span>
         <button class="done-btn bg-green-500 text-white px-2 py-1 rounded">Done</button>
       </div>
    ))}
  </div>
</div>
)
}

export default RenderBlob;

const data2 = [{ id: 1, message: "Server down" }, { id: 2, message: "Low memory" }]
function RenderData() {
const [notifications, setNotifications] = useState(data2)

  return (
    <div class="notifications bg-gray-200 p-6">
      <h1 class="text-xl font-semibold">Alerts</h1>
      <ul class="alert-list">
        {notifications.map((notifs) => (
           <li  key={notifs.id}class="alert bg-red-100 p-3 mb-2 rounded">{notifs.message}
           </li>
        ))}
     </ul>
</div>
  )
}

export default RenderData;



import react from 'react';
import { useState } from 'react';

const comment = [{ id: 1, text: "Great post!", author: "User1" }, { id: 2, text: "Thanks!", author: "User2" }]
function Comment ({ comment }) {
const [userComment, setComment] = useState(comment);

  return (
    <section class="comments p-4">
  <h2 class="text-2xl mb-4">Comments</h2>
  <div class="comment-thread">
    {userComment.map((user)=> (
      <article  key={user.id}class="comment bg-white p-3 mb-3 shadow">
       <p class="text-gray-800">{user.text}</p>
       <span class="text-gray-500">{user.author}</span>
     </article>
    ))}
  </div>
</section>
  )
}

export default Comment;

const data = [{ id: 1, name: "Meeting", date: "Mar 20" }, { id: 2, name: "Review", date: "Mar 21" }]
function LastRender () {
  const [events, setEvent] = useState(data);
    return (
      <div className="events bg-blue-100 p-5">
      <h3 className="text-lg font-bold">Upcoming Events</h3>
      <div className="event-list flex flex-col gap-3">
        {events.map((d_event) => (
          <div className="event bg-white p-4 rounded" key={d_event.id}>
          <span className="event-name">{d_event.name}</span>
          <span className="event-date">{d_event.date}</span>
        </div>
        ))}
      </div>
</div>
    )
}

export default LastRender;
