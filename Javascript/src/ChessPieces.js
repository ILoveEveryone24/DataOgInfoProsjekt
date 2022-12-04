import React, {useRef, useState} from "react";
import ChessBoard from "./ChessBoard";
import {ChessBoardList} from "./ChessBoard";

import availableMove from "./images/GreenCircle.svg";



export default function renderPieces({pieces})
{
	let showMoves = [];
	for(let i = 0; i < 64; i++)
	{
		showMoves.push("hidden");
	}

	const [piecesPos, setPiecesPos] = useState(pieces[0]);
	const [data, setData] = useState(showMoves);

	let availableMoveList = [];

	
	
	 	for(let i = 0; i < 8; i++)
	 	{
	 		
	 		for(let n = 0; n < 8; n++)
	 		{
	 			availableMoveList.push(
	 				<img 
	 				draggable = {false}
	 				key = {(i*10) + n + 300} 
	 				src={availableMove}
	 				onDragOver={dragOverPiece}
	 				onDrop = {dropped} 
	 				height="30" 
	 				width="30" 
	 				style=
	 					{{
	 						position:"absolute", 
	 						top:100 * n + 43, 
	 						left:100 * i + 43,
	 						visibility:data[n + (i * 8)]
	 					}}
	 				/>);
	 		}
	 	}


	function dragOverPiece(e)
	{
		e.preventDefault();
	}

	function dropped(e)
	{
		e.preventDefault();
		var pieceData = e.dataTransfer.getData("text");
		//pieces[0][pieceData].x = 200;
		const temp = piecesPos;
		for(const x in temp)
		{
			if(x === pieceData)
			{
				piecesPos[x] = {...piecesPos[x], y:500};
			}
		}
		console.log(temp);
		console.log(piecesPos);
		setPiecesPos(temp);
	}

    function dragPiece(e)
    {
    	e.dataTransfer.setData("text", e.target.id);
    	//console.log(data);
    	let changeData = [];
    	
    	for(let i = 0; i < 64; i++)
    	{
    		if((parseInt(e.target.style.top) === availableMoveList[i].props.style.top -43 + 100) && (parseInt(e.target.style.left) === availableMoveList[i].props.style.left -43))
    		{
    			changeData.push("visible");
    		}
    		else
    		{
    			changeData.push("hidden");
    		}
    		
    	}
    	//console.log(parseInt(e.target.style.top));
    	//console.log(availableMoveList[20].props.style.top -43 -100);
    	setData(changeData);
    }

	return(
		<div>
		{availableMoveList}
		{
            Object.entries(piecesPos).map(([key, value]) => {
                return(
                    <img src={value.src}
                    id = {key}
                    draggable = {true}
                    onDragStart = {dragPiece}
                    key = {key}
                    height="100"
                    width="100"
                    style={{position:"absolute", top:value.y, left:value.x}}
                    />
                )
            })
        }



		</div>
	)
}