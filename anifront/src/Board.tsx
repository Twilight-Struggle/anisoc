import React, { useState } from "react";
import zibun_saru from "./images/zibun_saru.png";

type PropsSquare = {
  onClick: () => void;
};
function Square(props: PropsSquare) {
  // onClickでmove_listに自身のkeyを追加
  return (
    <button onClick={props.onClick} className="square">
      <img src={zibun_saru} />
    </button>
  );
}

type PropsBoard = {
  handleMoveInput: (index: number) => void;
};
function Board(props: PropsBoard) {
  const renderSquare = (i: number) => {
    return <Square onClick={() => props.handleMoveInput(i)} />;
  };
  return (
    <div>
      <div className="board-row">
        {renderSquare(0)}
        {renderSquare(1)}
        {renderSquare(2)}
      </div>
      <div className="board-row">
        {renderSquare(3)}
        {renderSquare(4)}
        {renderSquare(5)}
      </div>
      <div className="board-row">
        {renderSquare(6)}
        {renderSquare(7)}
        {renderSquare(8)}
      </div>
      <div className="board-row">
        {renderSquare(9)}
        {renderSquare(10)}
        {renderSquare(11)}
      </div>
      <div className="board-row">
        {renderSquare(12)}
        {renderSquare(13)}
        {renderSquare(14)}
      </div>
    </div>
  );
}

export default Board;
