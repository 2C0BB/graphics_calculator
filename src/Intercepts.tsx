import { useEffect, useState } from "react";

function Intercepts ({
	eq1,
	setEq1,

	eq2,
	setEq2,

	steps,
	setSteps,

	epsilon,
	setEpsilon,

	intercepts
}: {
	eq1: any,
	setEq1: any,

	eq2: any,
	setEq2: any,

	steps: number,
	setSteps: any,

	epsilon: number,
	setEpsilon: any,

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

			<input type="number"
				value={steps}
				onChange={(e) => setSteps(e.target.value)}
			/>
			<input type="number"
				value={epsilon}
				onChange={(e) => setEpsilon(e.target.value)}
			/>

			<br></br>
			<b>{intercepts.length}</b>

			{
				intercepts.map((i: number[], idx: number) => {
					return <div key={idx}><p>{i[0].toFixed(4)} : {i[1].toFixed(4)}</p></div>;
				})
			}
		</>
	);
}

export default Intercepts;