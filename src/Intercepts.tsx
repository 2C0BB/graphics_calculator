import { useEffect, useState } from "react";

function Intercepts ({
	eq1,
	setEq1,

	eq2,
	setEq2
}: {
	eq1: any,
	setEq1: any,

	eq2: any,
	setEq2: any
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

			<p></p>
		</>
	);
}

export default Intercepts;