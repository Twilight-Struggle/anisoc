import React, { useState } from "react";
import zibun_saru from "./images/zibun_saru.png";
import zibun_oyasaru from "./images/zibun_oyasaru.png";
import zibun_risu from "./images/zibun_risu.png";
import zibun_usagi from "./images/zibun_usagi.png";
import aite_saru from "./images/aite_saru.png";
import aite_oyasaru from "./images/aite_oyasaru.png";
import aite_risu from "./images/aite_risu.png";
import aite_usagi from "./images/aite_usagi.png";
import ball_img from "./images/ball.png";

type PropsSquare = {
  onClick: () => void;
  image: string | null;
};
function Square(props: PropsSquare) {
  // onClickでmove_listに自身のkeyを追加
  if (props.image != null) {
    return (
      <button onClick={props.onClick} className="square">
        <img src={props.image} alt="" />
      </button>
    );
  } else {
    return <button onClick={props.onClick} className="square"></button>;
  }
}

type PropsBoard = {
  handleMoveInput: (index: [number, number]) => void;
  board: (string | null)[][];
};
function Board(props: PropsBoard) {
  const renderSquare = (i: number) => {
    const index: [number, number] = [Math.floor(i / 3), i % 3];
    let image_path: string | null;
    switch (props.board[index[0]][index[1]]) {
      case "O_A":
        image_path = zibun_oyasaru;
        break;
      case "S_A":
        image_path = zibun_saru;
        break;
      case "R_A":
        image_path = zibun_risu;
        break;
      case "U_A":
        image_path = zibun_usagi;
        break;
      case "O_D":
        image_path = aite_oyasaru;
        break;
      case "S_D":
        image_path = aite_saru;
        break;
      case "R_D":
        image_path = aite_risu;
        break;
      case "U_D":
        image_path = aite_usagi;
        break;
      case "B_N":
        image_path = ball_img;
        break;
      default:
        image_path = null;
    }
    return (
      <Square onClick={() => props.handleMoveInput(index)} image={image_path} />
    );
  };
  return (
    <div>
      <div className="board-row">
        <button className="goal" onClick={() => props.handleMoveInput([5, 0])}>
          Goal
        </button>
        <button className="goal" onClick={() => props.handleMoveInput([5, 1])}>
          Goal
        </button>
        <button className="goal" onClick={() => props.handleMoveInput([5, 2])}>
          Goal
        </button>
      </div>
      <div className="board-row">
        {renderSquare(12)}
        {renderSquare(13)}
        {renderSquare(14)}
      </div>
      <div className="board-row">
        {renderSquare(9)}
        {renderSquare(10)}
        {renderSquare(11)}
      </div>
      <div className="board-row">
        {renderSquare(6)}
        {renderSquare(7)}
        {renderSquare(8)}
      </div>
      <div className="board-row">
        {renderSquare(3)}
        {renderSquare(4)}
        {renderSquare(5)}
      </div>
      <div className="board-row">
        {renderSquare(0)}
        {renderSquare(1)}
        {renderSquare(2)}
      </div>
      <div className="board-row">
        <button className="goal" onClick={() => props.handleMoveInput([-1, 0])}>
          Goal
        </button>
        <button className="goal" onClick={() => props.handleMoveInput([-1, 1])}>
          Goal
        </button>
        <button className="goal" onClick={() => props.handleMoveInput([-1, 2])}>
          Goal
        </button>
      </div>
    </div>
  );
}

export default Board;
