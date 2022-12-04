import React, {useState} from "react";
import ChessBoard from "./ChessBoard";
import ChessPieces from "./ChessPieces";
import Chat from "./Chat";

import io from "socket.io-client";

import {DefaultPieces} from "./DefaultPiecesLayout";

const socket = io.connect("http://localhost:8000/websocket");

const App = () =>{
    const [piecesMovement] = useState([DefaultPieces]);

    return (
        <>
     {  <ChessBoard/>            
            <ChessPieces pieces = {piecesMovement}/>}
            //<Chat socket = {socket}/>
        </>
    )
}

export default App;
