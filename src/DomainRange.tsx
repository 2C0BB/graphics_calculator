import { useEffect, useState } from "react";

function DomainRange ({
	minX,
	setMinX,

	maxX,
	setMaxX,

	minY,
	setMinY,

	maxY,
	setMaxY,
}: {
	minX: number,
	setMinX: any,

	maxX: number,
	setMaxX: any,

	minY: number,
	setMinY: any,

	maxY: number,
	setMaxY: any
}) {

	return (
		<>
			<div className="domainRange">
			<label>x-min: </label>
			<input
				type="number"
				value={minX}
				onChange={(e) => setMinX(e.target.value)}
			/>
			<br />
			<label>x-max: </label>
			<input
				type="number"
				value={maxX}
				onChange={(e) => setMaxX(e.target.value)}
			/>

			<br />
			<br />

			<label>y-min: </label>
			<input
				type="number"
				value={minY}
				onChange={(e) => setMinY(e.target.value)}
			/>
			<br />
			<label>y-max: </label>
			<input
				type="number"
				value={maxY}
				onChange={(e) => setMaxY(e.target.value)}
			/>
			</div>
		</>
	);

}

export default DomainRange;