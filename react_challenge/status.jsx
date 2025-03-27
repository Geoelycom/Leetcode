import react from 'react';
import { useState } from 'react';


function StatusList() {
  const [statuses, setStatuses] = useState(["on", "off"]);
  const toggleStatus = (index) => { 
    const newStatus = [...statuses]; 
    newStatus[index] = newStatus[index] === "on" ? "off" : "on";
    setStatuses(newStatus);
  }

  const statusList = statuses.map((s, i) => {
    return (
      <div>
        <Status state={s} key={i}/>
        <button onClick={toggleStatus(i)}>Toggle </button>
      </div>
    );
  });
  return <div>{statusList}</div>;
}

function Status({ state }) {
  return <p> Status:{state}</p>
}

export default StatusList;



// Counter Grid

function CounterGrid() {
  const [counters, setCounters] = useState([0, 0, 0]);

  const incrementCounter = (index) => {
    const newCounter = [...counters];
    newCounter[index]= setCounters[index] + 1;

  }

    const decrementCounter = (index) => {
      const newCounter = [...counters];
      if (newCounter[index] > 0) {
        newCounter[index] = setCounters[index] - 1;
      }
      setCounters(newCounter);
    }
    
    const Counterlist = counters.map((c , i) => {
      return (
        <div>
          <Counter value={c} key={i} />
          <button onClick={() => incrementCounter(i)}>Increment</button>
          <button onClick={() => decrementCounter(i)}>Decrement</button>
        </div>
      )
    })
    return <div>{Counterlist}</div>;
  }

function Counter({ value }) {
  return <p>Count: {value}</p>
}


export default CounterGrid;









function ScoreGrid() {
  const [scores, setScores] = useState([10, 20]);

  const incrementScore = (index) => {
    setScores(scores.map((s, i) => 
      i === index && s < 30 ? s + 5 : s ));
  }
  const scoreList = scores.map((s, i ) => {
    <div>
      <Score points={s} key={i}/>
      <button onClick={() => incrementScore()}>Add 5 </button>
    </div>
  })

  return <div>{scoreList}</div>
}

function Score ({ points }) {
  return <p>Score: {points}</p>;
}

export default ScoreGrid;




function TempCounter() {
  const [temp, setTemp] = useState(10);

  const incrementTemp = () => {
    if (temp < 30) { // Same max logic
      setTemp(temp + 5);
    }
  };

  function TempCounter () {
    const [ temp, setTemp] = useState(10);
    if (temp < 30 ) {
      setTemp((prevTemp ) => prevTemp + 5);
    }
  }
}



  import React, { useState } from 'react';

function Counter() {
  const [count, setCount] = useState(0);
  const increment = () => {
    if (count < 15 ) {
      setCount((prevCount) => prevCount + 3);
  }
}
  return (
    <div>
      <Display count={count} />
      <button onClick={() => increment()}>Increment</button>
      {/* Count + Button */}
    </div>
  )
  
}

function Display({ value }) {
  return <p>Count: {value}</p>;
}




import React, { useState } from 'react';

function ScoreCounter() {
  const [score, setScore] = useState(0);
  const incrementScore = () => {
    if (score < 12 ) {
      setScore ((prevScore) => prevScore + 4 );
    }
  }
  // Your code
  return (
  <div>
    <Score points={score} />
    <button onClick={() => incrementScore()}>Add 4</button>
    {/* Child + Button */}
    </div>
    );
}

function Score({ points }) {
  return <p>Score: {points}</p>;
}



import React, { useState } from 'react';

function LevelCounter() {
  const [level, setLevel] = useState(1);
  const updateLevel = () => {
    if (level < 9 ) {
      setLevel((prevLevel) => prevLevel + 2 )
    }
  }

  const decrementLevel = () => {
    if (level > 1 ) {
      setLevel((prevLevel) => prevLevel - 2);
    }
  }
  return (
    <div>
      <Rank rank={level} />
      <button onClick={() => updateLevel()}>Increase</button>
      <button onClick={() => decrementLevel()}>Decrease</button>
      {/* Child + Buttons */}
    </div>
  )
}

function Rank({ rank }) {
  return <p>Level: {rank}</p>;
}

export default LevelCounter;



// Task: [value, setValue] = useState(0), “Add” (+1, no args), “Set” (to 5), prop num.
// destructing states in react is used when working with multiple values in state, when its a single value we can just use useState(0) and when its multiple values we can use useState({value: 0, name: 'John'}) and then we can use it as value.value and value.name


import React, { useState } from 'react';

function ValueCounter() {
  const [value, setValue] = useState(0);
  // Your code
  const updateValue = () => {
    setValue((prevValue) => prevValue + 1);
  }

  const setValueToFive = () => {
    setValue(5);
  }

  return( 
   <div>
   <Value num={value} />
    <button onClick={updateValue}>Add</button>
    <button onClick={() => setValueToFive()}>Set</button>
    </div>
  );
}

function Value({ num }) {
  return <p>Value: {num}</p>;
}


function Timer() {
  const [timer, setTimer] = useState(10);
  const updateTimer = () => {
    if (timer > 0 ) {
      setTimer((prevTimer) => prevTimer - 1);
    }
  }

  const resetTimer = () => {
    setTimer(10);
  }

  return (
    <div>
      <Clock time={timer} />
      <button onClick={updateTimer}>Update</button>
      <button onClick={resetTimer}>Reset</button>
    </div>
  )
}


function Clock({ time }) {
  return <p>Time: {time}</p>;
}


// export default Timer;


import React, { useState } from 'react';

function TaskTimer() {
  const [time, setTime] = useState({ hours: 0, minutes: 0 });

  const incrementHours = () => {
    setTime((prevTime) => ({ 
      ...prevTime,
      hours: (prevTime.hours + 1) % 24
    }))
  };


  const incrementMinutes = () => {
    setTime((prevTime) => ({ 
      ...prevTime,
      hours: (prevTime.hours + 1) % 60
  }))


  const decrementHours = () => {
    setTimer((prevTime) => ({ 
      ...prevTime,
      hours: prevTime.hours - 1 < 0 ? 23 : prevTime.hours - 1
    }))
  };


  const decrementMinutes = () => {
    setTimer((prevTime) => ({ 
      ...prevTime,
      minutes: prevTime.minutes - 1 < 0 ? 59 : prevTime.minutes - 1
    }))
  };
  const reset = () => {
    setTime({ hours: 0, minutes: 0 });
  };

  // Your code here

  return (
    <div id="TaskTimer" className="container">
     <div class="row">
        <button id="hours-up-button" onClick={incrementHours}>↑</button>
        <button id="minutes-up-button" onClick={incrementMinutes}>↑</button>
     </div>


    <div class="row">
      <div id="timer-display">
        <TaskDisplay duration={time} />
      </div>
    </div>

      <div class="row">
        <button id="hours-down-button" onClick={decrementHours}>↓</button>
        <button id="minutes-down-button" onClick={decrementMinutes}>↓</button>
        <button id="reset-button" onClick={reset}>Reset</button>
      </div>
      
    </div>
  );
 }
}

function TaskDisplay({ duration }) {
  // Format as HH:MM (e.g., "03:45")
  const formattedTime = `${String(duration.hours).padStart(2, '0')}:${String(duration.minutes).padStart(2, '0')}`;
  return (
    <div id="timer-display">
      {formattedTime}
    </div>
  )
}

export default TaskTimer;




// const [fruits, setFruits] = useState(["apple", "banana", "cherry"]);

// const selectFruits = (index, newFruit) => {
//   let newFruits = [...fruits];
//   setFruits(())
// }

// NOTES ON ARRAY OF STATES
// 1. We use .map to update specific items.
// 2. we use [...fruits, newItem] to add a new item(whe we use destructuring).
// 3. we use .filter to remove an item.

// UPDATING PAGES USING API REQUESTS DATA. (FETCH / AXIOUS)

const fetchImages = async () => {
  const response = await fetch()
}