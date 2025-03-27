// Challenge: Score Tracker
// State: [scores, setScores] = useState([])—array of { id, player, score }.
// Actions:
// “Add Player” (id: add-button)—adds { id: scores.length + 1, player: "Player {id}", score: 0 }.
// “Add Point” (id: point-${id})—increments score.
// Display: <Player>—prop player.


import React, { useState } from 'react';

function ScoreTracker() {
  const [scores, setScores] = useState([]);

  const addPlayer = () => {
    setScores((prevScores) => [
      ...prevScores,
      {
        id: prevScores.length + 1,
        player: `Player ${prevScores.length + 1}`,
        score: 0
      }
    ])
  }

  const addPoint = (id) => {
    setScores((prevScores) => prevScores.map((player) => {
      if (id === player.id) {
        return {
          ...player,
          score: player.score + 1
        }
      }
      return player;
    }))
  }

  return (
    <div id="ScoreTracker" className="container">
      <div className="row">
        <button id="add-button" onClick={addPlayer}>Add Player</button>
      </div>
      <div className="row" id="scores">
        {/* Render scores */}
        {scores.map((player) => (
          <Player key={player.id} player={player} onPoint={() => addPoint(player.id)} />
        ))}
      </div>
    </div>
  );
}

function Player({ player, onPoint }) {
  return (
    <div>
      <span>{player.player} - {player.score}</span>
      <button id={`point-${player.id}`} onClick={onPoint}>Add Point</button>
    </div>
  );
}

export default ScoreTracker;