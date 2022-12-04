import React, {useState} from "react";



export default function ChessBoard(){    

    const[color, setColor] = useState(["#cd9575", "#3d2b1f"]);

    const ChessBoardList = [];
    let layout = [0,1,0,1,0,1,0,1];
    
    for(let i = 0; i < 8; i++)
    {
        for(let n = 0; n < 8; n++)
        {
            ChessBoardList.push(<rect 
                key = {(i*10) + n} 
                x={n * 100}
                y={i * 100}
                width="100"
                height = "100"
                fill={color[layout[n]]}/>)
        }
        if(i % 2)
        {
            layout = [0,1,0,1,0,1,0,1];
        }
        else
        {
            layout = [1,0,1,0,1,0,1,0];
        }
    }
    
    	return(
            <svg width="800" height="800" viewport="0 0 800 800">
    		{
                ChessBoardList.map(BoardPiece =>{
                    return(BoardPiece)
                })
            }
            </svg>
    	)
}
