import React, { useState } from 'react';
import PriorityBlock from '../../../../PriorityBlock';
const renderList = () => {
  const [task, setTask] = useState([PriorityBlock]);

  return (
    <div>
      <h2> Book List</h2>
      <button onClick={ClearCompleted}>Clear Completed</button>
      <div>
        {task.map((task) => (
          <div key={task.id}>
            <p>{task.title}</p>
            <p>{task.description}</p>
          </div>
        ))}
      </div>
    </div>
  )
}
export default renderList;



{/* <article className="customers">
  {data.map((customer) => (
    <div key={customer.id} className="customer-card">
      <h3>{customer.firstName} {customer.lastName}</h3>

      <div className="customer__stats">
        <div className="stats">
          <span className="stats__number">{customer.loyaltyPoints}</span>
          <span className="stats__description">Loyalty Points</span>
        </div>
        <div className="stats">
          <span className="stats__number">{customer.numberOfPurchases}</span>
          <span className="stats__description">Purchases</span>
        </div>
      </div>

      <h4 className="customer__key">Email</h4>
      <span className="customer__value customer__email">{customer.email}</span>

      <h4 className="customer__key">Phone number</h4>
      <span className="customer__value customer__phone-number">{customer.phoneNumber}</span>

      <h4 className="customer__key">Personal Manager</h4>
      <span className="customer__value customer__personal-manager">{customer.personalManager}</span>
    </div>
  ))}
</article> */}

const RenderList = () => {
  const [user, setUser] = useState([]); // pass the array inside the usestate
  
  return (
    <div> 
      {user.map((users) =>(
        <div className="card" key={users.id}>
        <p>{users.name}</p>
        <p>{users.email}</p>
        <p>{users.phone}</p>
        </div>
      ) )}
    </div>
  )
}

export default RenderList;

function fetchpost() {

   const [product] = useState(productarray)
  return (
    product.map((data) => (
      
    ))
  )
}

// event handler for search items
const handlesearch = (e) => {
  setSearchItem(e.target.value);
}

// filter post based on seach in input
const [product, setProduct] = ("");
const filteredPosts = () => {
  product.filter((item) => {
    item.title.toLowercase().includes(setProduct.toLowercase())
  })
}

// delete a particular post
const handleDelete = (id) => {
  setProduct((prevsProduct) => prevsProduct.filter((item) => item.id !== id))
}


function XAI () {
  const [customers, setCustomers] = useState(data);


  return (
    <div className="customer-card">
      {data.map((user) => (

      ))}

    </div>
  )
}