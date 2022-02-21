import React from "react";
import { rest } from "msw";
import { setupServer } from "msw/node";
import { render, screen, fireEvent } from "@testing-library/react";
import App from "./App";

const server = setupServer(
  rest.post("/make", (req, res, ctx) => {
    return res(
      ctx.json({
        res: "made",
        id: 9999,
        board: [
          ["U_A", null, "R_A"],
          [null, "S_A", null],
          [null, "B_N", null],
          [null, "S_D", null],
          ["R_D", null, "U_D"],
        ],
      })
    );
  }),
  rest.post("/reset", (req, res, ctx) => {
    return res(
      ctx.json({
        res: "reset",
        id: 9999,
        board: [
          ["U_A", null, "R_A"],
          [null, "S_A", null],
          [null, "B_N", null],
          [null, "S_D", null],
          ["R_D", null, "U_D"],
        ],
      })
    );
  }),
  rest.post("/mov", (req, res, ctx) => {
    return res(
      ctx.json({
        res: "Game Continues",
        id: 9999,
        board: [
          [null, "U_A", "R_A"],
          [null, "S_A", null],
          [null, "B_N", null],
          [null, "S_D", null],
          ["R_D", null, "U_D"],
        ],
      })
    );
  })
);

beforeAll(() => server.listen());
afterEach(() => server.resetHandlers());
afterAll(() => server.close());

test("move click test", () => {
  render(<App />);
  fireEvent.click(screen.getByTestId("board4"));
  expect(screen.getByLabelText("moves")).toHaveTextContent("(1,1)");
  fireEvent.click(screen.getByTestId("board7"));
  fireEvent.click(screen.getByText("移動キャンセル"));
  expect(screen.getByLabelText("moves")).toHaveTextContent("");
});

test("make test", async () => {
  render(<App />);
  fireEvent.click(screen.getByText("作成"));
  expect(await screen.findByText("made")).toBeInTheDocument();
  expect(screen.getByTestId("board4_img")).toBeInTheDocument();
  expect(screen.queryByTestId("board3_img")).toBeNull();
});

test("reset test", async () => {
  render(<App />);
  fireEvent.click(screen.getByText("作成"));
  await screen.findByText("made");
  fireEvent.click(screen.getByText("リセット"));
  expect(await screen.findByText("reset")).toBeInTheDocument();
  expect(screen.getByTestId("board4_img")).toBeInTheDocument();
  expect(screen.queryByTestId("board3_img")).toBeNull();
});

test("mov test", async () => {
  render(<App />);
  fireEvent.click(screen.getByText("作成"));
  await screen.findByText("made");
  fireEvent.click(screen.getByTestId("board0"));
  fireEvent.click(screen.getByTestId("board1"));
  fireEvent.click(screen.getByText("移動実行"));
  expect(await screen.findByText("Game Continues")).toBeInTheDocument();
  expect(screen.getByTestId("board1_img")).toBeInTheDocument();
  expect(screen.queryByTestId("board0_img")).toBeNull();
});
