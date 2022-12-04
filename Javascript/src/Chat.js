import React from "react";



export default function Chat({socket}){ 

	socket.on("receive_message", (data) => {
		console.log(data);
	});

	return(
		<div>
			<p>{"Hello"}</p>
		</div>
	)
}