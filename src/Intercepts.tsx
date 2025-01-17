import { useEffect, useState } from "react";

function Intercepts ({
	eq1,
	setEq1,

	eq2,
	setEq2,

	intercepts
}: {
	eq1: any,
	setEq1: any,

	eq2: any,
	setEq2: any,

	intercepts: number[][]
}) {



	return (
		<>
			<input 
				type="text"
				value={eq1}
				onChange={(e) => setEq1(e.target.value)}
			/>
			<input 
				type="text"
				value={eq2}
				onChange={(e) => setEq2(e.target.value)}
			/>

			<br></br>
			<b>Num of intercepts: {intercepts.length}</b>

			{
				intercepts.map((i: number[], idx: number) => {
					return <div key={idx}><p>({i[0].toFixed(4)}, {i[1].toFixed(4)})</p></div>;
				})
			}
		</>
	);
}

export default Intercepts;