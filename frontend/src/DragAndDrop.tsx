import { createSignal, For, Show } from "solid-js";

export const DragAndDrop = () => {
	const [dragging, setDragging] = createSignal<number | null>(null);
	const [target, setTarget] = createSignal<number | null>(null);
	const [items, setItems] = createSignal([
		"item0",
		"item1",
		"item2",
		"item3",
		"item4",
		"item5",
		"item6",
		"item7",
	]);

	const reset = () => {
		setItems([
			"item0",
			"item1",
			"item2",
			"item3",
			"item4",
			"item5",
			"item6",
			"item7",
		]);
	};

	const handleDragStart = (e: DragEvent) => {
		setDragging(parseInt((e.target as HTMLLIElement).dataset.index!));
	};

	const handleDragEnd = (e: DragEvent) => {
		setDragging(null);
		setTarget(null);
	};

	const handleDrop = (e: DragEvent) => {
		e.preventDefault();
		const from = dragging()!;
		const to = parseInt((e.target as HTMLLIElement).dataset.index!);
		const updated = [...items()];
		const [item] = updated.splice(from, 1);
		updated.splice(from < to ? to - 1 : to, 0, item);
		console.log(updated);
		setItems(updated);
	};

	return (
		<div
			onDrag={(e) => console.log(e)}
			onDragStart={handleDragStart}
			onDragEnd={handleDragEnd}
			onDragEnter={(e) => {
				e.preventDefault();
				const index = (e.target as HTMLLIElement).dataset.index;
				setTarget(parseInt(index!));
			}}
			onDragOver={(e) => e.preventDefault()}
			onDragLeave={(e) => {
				e.preventDefault();
				const index = (e.target as HTMLLIElement).dataset.index;
				if (target() === parseInt(index!)) setTarget(null);
			}}
			onDrop={handleDrop}
		>
			<button onClick={reset}>reset</button>
			<ul>
				<For each={items()}>
					{(item, idx) => (
						<li
							data-index={idx()}
							draggable="true"
							classList={{
								"dnd-hover": target() === idx(),
								"dnd-dragging": dragging() === idx(),
							}}
						>
							{item}
						</li>
					)}
				</For>
			</ul>
		</div>
	);
};
