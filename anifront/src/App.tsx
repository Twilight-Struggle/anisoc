import React, { useState } from "react";
import Board from "./Board";

function App() {
  const [made, setMade] = useState<boolean>(false);
  const [status, setStatus] = useState("こんにちは");
  const [gameid, setGameid] = useState("");
  const [board, setBoard] = useState<(string | null)[][]>(
    [...Array(5)].map(() => Array(3).fill(null))
  );
  // move_list:配列宣言
  const [moves, setMoves] = useState<[number, number][]>([]);
  // push_move() move_listに引数をプッシュ
  const handleMoveInput = (index: [number, number]) => {
    if (moves.length <= 3) {
      const new_moves = moves.slice();
      new_moves.push(index);
      setMoves(new_moves);
    }
  };
  // move_cancell() 配列の中身クリア
  const handleMoveCancel = () => {
    const new_moves: [number, number][] = [];
    setMoves(new_moves);
  };
  // move_exec() moveを送信し配列の中身クリア 帰ってきた値を反映
  const handleMovePush = () => {
    if (moves.length === 2 || moves.length === 3) {
      let requestOption;
      if (moves.length === 2) {
        requestOption = {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            id: gameid,
            act: { from: moves[0], to: moves[1], kickto: null },
          }),
        };
      } else {
        requestOption = {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            id: gameid,
            act: { from: moves[0], to: moves[1], kickto: moves[2] },
          }),
        };
      }
      fetch("/mov", requestOption)
        .then((res) => res.json())
        .then((data) => {
          setStatus(data.res);
          setBoard(data.board);
        });
    } else {
      setStatus("illegal input");
    }
    handleMoveCancel();
  };
  // make() バックエンドへ作成を依頼
  const handleMake = () => {
    fetch("/make", { method: "POST" })
      .then((res) => res.json())
      .then((data) => {
        setStatus(data.res);
        setGameid(data.id);
        setBoard(data.board);
      });
    setMade(true);
  };
  // reset() バックエンドへリセットを依頼
  const handleReset = () => {
    const requestOption = {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ id: gameid }),
    };
    fetch("/reset", requestOption)
      .then((res) => res.json())
      .then((data) => {
        setStatus(data.res);
        setBoard(data.board);
      });
  };
  return (
    <div className="app">
      <div className="board">
        <Board handleMoveInput={handleMoveInput} board={board} />
      </div>
      <div className="info">
        {made ? (
          <button onClick={handleReset}>リセット</button>
        ) : (
          <button onClick={handleMake}>作成</button>
        )}
        <button onClick={handleMovePush}>移動実行</button>
        <button onClick={handleMoveCancel}>移動キャンセル</button>
        <label>{status} </label>
        <label aria-label="moves">
          {moves[0] && "("}
          {moves[0] && moves[0][0]}
          {moves[0] && ","}
          {moves[0] && moves[0][1]}
          {moves[0] && ")"}
          {moves[1] && ","}
          {moves[1] && "("}
          {moves[1] && moves[1][0]}
          {moves[1] && ","}
          {moves[1] && moves[1][1]}
          {moves[1] && ")"}
          {moves[2] && ","}
          {moves[2] && "("}
          {moves[2] && moves[2][0]}
          {moves[2] && ","}
          {moves[2] && moves[2][1]}
          {moves[2] && ")"}
        </label>
      </div>
    </div>
  );
}

export default App;
