import {
	createSignal,
	type JSX,
	type JSXElement,
	onCleanup,
	onMount,
	Show,
	type ValidComponent,
} from "solid-js";
import { Portal, render } from "solid-js/web";
import {
	autoUpdate,
	flip,
	offset,
	type Placement,
	shift,
} from "@floating-ui/dom";
import { useFloating } from "solid-floating-ui";

// WARNING: this is potentially very laggy
// TODO: defer tooltip
type TooltipProps = {
	tipText?: string;
	attrs?: Record<string, string>;
	interactive?: boolean;
	placement?: Placement;
	animGroup?: string;
	doesntRetain?: string;
	mount?: HTMLElement;

	// https://floating-ui.com/docs/detectoverflow#altboundary
	altBoundary?: boolean;
};

type TooltipAnimState = {
	shouldAnim: boolean;
	timeout: NodeJS.Timeout;
};

const tooltipAnimSuppress = new Map<string, TooltipAnimState>();

// TODO: only use one tooltip + event listener instead of per element
// or, debug performance issues in geneal
export function tooltip(
	props: TooltipProps,
	tip: ValidComponent,
	wrap: HTMLElement,
) {
	const [contentEl, setContentEl] = createSignal<HTMLElement>();
	const [tipEl, setTipEl] = createSignal<HTMLDivElement>();
	// const [title, setTitle] = createSignal(getTitle());
	const [visible, setVisible] = createSignal(false);
	const [animate, setAnimate] = createSignal(true);
	let popupRemoveTimeout: NodeJS.Timeout;
	let isHovered = false;
	const overlayEl = document.getElementById("overlay")!;
	const padding = () => 8;

	if (props.animGroup) {
		const s = tooltipAnimSuppress.get(props.animGroup);
		if (!s) {
			tooltipAnimSuppress.set(props.animGroup, {
				shouldAnim: true,
				timeout: 0 as unknown as NodeJS.Timeout,
			});
		}
	}

	function getTitle() {
		return props.tipText ?? (typeof tip === "string" ? tip : "");
	}

	function showTip() {
		clearTimeout(popupRemoveTimeout);
		isHovered = true;
		if (visible()) return;
		if (props.animGroup) {
			const s = tooltipAnimSuppress.get(props.animGroup)!;
			// console.log(s);
			setAnimate(s.shouldAnim);
			s.shouldAnim = false;
			clearTimeout(s.timeout);
		}
		setVisible(true);
		wrap.title = "";
	}

	function hideTip() {
		// TODO: exit animations? might be too much
		wrap.title = getTitle();
		setVisible(false);
		if (props.animGroup) {
			const s = tooltipAnimSuppress.get(props.animGroup)!;
			s.timeout = setTimeout(() => {
				s.shouldAnim = true;
			}, 500);
		}
		isHovered = false;
	}

	function considerHidingTip() {
		// FIXME: nested popups/tooltips can cause issues with isHovered
		// maybe have global set of what is hovered and what is a parent of what
		isHovered = false;
		if (!props.interactive) return hideTip();
		if (
			props.doesntRetain && document.activeElement?.matches(props.doesntRetain)
		) return hideTip();
		if (tipEl()?.contains(document.activeElement)) return;
		popupRemoveTimeout = setTimeout(hideTip, 0);
	}

	function showTipIfInteractive() {
		if (props.interactive) showTip();
	}

	function handleFocusOff() {
		if (!isHovered) hideTip();
	}

	const pos = useFloating(contentEl, tipEl, {
		whileElementsMounted: autoUpdate,
		strategy: "fixed",
		placement: props.placement,
		// HACK: make volume slider work properly
		middleware: props.placement === "top-start"
			? [shift({ padding: padding() }), offset({ mainAxis: -8 }), flip()]
			: [shift({ padding: padding() }), offset({ mainAxis: 8 }), flip()],
	});

	wrap.addEventListener("mouseenter", showTip);
	wrap.addEventListener("mouseleave", considerHidingTip);
	setContentEl(wrap);

	onCleanup(() => {
		wrap.addEventListener("mouseenter", showTip);
		wrap.addEventListener("mouseleave", considerHidingTip);
	});

	// TODO: use onPointerEnter/Leave instead of mouse events?
	return (
		<>
			{wrap}
			<Show when={visible()}>
				<Portal mount={props.mount ?? overlayEl}>
					<div
						onMouseEnter={showTipIfInteractive}
						onMouseLeave={considerHidingTip}
						onFocusOut={handleFocusOff}
						ref={setTipEl}
						style={{
							position: pos.strategy,
							translate: `${pos.x}px ${pos.y}px`,
							visibility: visible() ? "visible" : "hidden",
							"--padding": `${padding()}px`,
						}}
						class="tooltip"
						classList={{
							animate: animate(),
							interactive: props.interactive,
						}}
					>
						<div class="base"></div>
						<div class="inner">{tip}</div>
					</div>
				</Portal>
			</Show>
		</>
	);
}

type CreateTooltipProps = Omit<TooltipProps, "mount"> & {
	tip: () => JSXElement;
	mount?: () => HTMLElement | undefined;
};

// TODO: only use one tooltip + event listener instead of per element
// or, debug performance issues in general
export function createTooltip(props: CreateTooltipProps) {
	const [contentEl, setContentEl] = createSignal<JSX.Element>();
	const [tipEl, setTipEl] = createSignal<JSX.Element>();
	// const [title, setTitle] = createSignal(getTitle());
	const [visible, setVisible] = createSignal(false);
	const [animate, setAnimate] = createSignal(true);
	let popupRemoveTimeout: NodeJS.Timeout;
	let isHovered = false;
	const overlayEl = document.getElementById("overlay")!;
	const padding = () => 8;

	if (props.animGroup) {
		const s = tooltipAnimSuppress.get(props.animGroup);
		if (!s) {
			tooltipAnimSuppress.set(props.animGroup, {
				shouldAnim: true,
				timeout: 0 as unknown as NodeJS.Timeout,
			});
		}
	}

	function getTitle() {
		const tip = tipEl();
		return props.tipText ?? (typeof tip === "string" ? tip : "");
	}

	function showTip() {
		clearTimeout(popupRemoveTimeout);
		isHovered = true;
		if (visible()) return;
		if (props.animGroup) {
			const s = tooltipAnimSuppress.get(props.animGroup)!;
			// console.log(s);
			setAnimate(s.shouldAnim);
			s.shouldAnim = false;
			clearTimeout(s.timeout);
		}
		setVisible(true);
		const wrap = contentEl();
		if (wrap instanceof HTMLElement) wrap.title = "";
	}

	function hideTip() {
		// TODO: exit animations? might be too much
		const wrap = contentEl();
		if (wrap instanceof HTMLElement) wrap.title = getTitle();
		setVisible(false);
		if (props.animGroup) {
			const s = tooltipAnimSuppress.get(props.animGroup)!;
			s.timeout = setTimeout(() => {
				s.shouldAnim = true;
			}, 500);
		}
		isHovered = false;
	}

	function considerHidingTip() {
		// FIXME: nested popups/tooltips can cause issues with isHovered
		// maybe have global set of what is hovered and what is a parent of what
		isHovered = false;
		if (!props.interactive) return hideTip();
		if (
			props.doesntRetain && document.activeElement?.matches(props.doesntRetain)
		) return hideTip();
		const tip = tipEl();
		if (tip instanceof Node && tip.contains(document.activeElement)) return;
		popupRemoveTimeout = setTimeout(hideTip, 0);
	}

	function showTipIfInteractive() {
		if (props.interactive) showTip();
	}

	function handleFocusOff() {
		if (!isHovered) hideTip();
	}

	const pos = useFloating(
		contentEl as () => HTMLElement,
		tipEl as () => HTMLElement,
		{
			whileElementsMounted: autoUpdate,
			strategy: "fixed",
			placement: props.placement,
			// HACK: make volume slider work properly
			middleware: [
				shift({ padding: padding(), altBoundary: props.altBoundary }),
				offset({ mainAxis: props.placement === "top-start" ? -8 : 8 }),
				flip(),
			],
		},
	);

	// TODO: use onPointerEnter/Leave instead of mouse events?
	// TODO: make typescript happy
	return {
		update: pos.update,
		content: (el: HTMLElement) => {
			onMount(() => {
				setContentEl(el);
				el.addEventListener("mouseenter", showTip);
				el.addEventListener("mouseleave", considerHidingTip);
			});

			onCleanup(() => {
				el.removeEventListener("mouseenter", showTip);
				el.removeEventListener("mouseleave", considerHidingTip);
			});

			render(() => {
				return (
					<Show when={visible()}>
						<Portal mount={props.mount?.() ?? overlayEl}>
							<div
								onMouseEnter={showTipIfInteractive}
								onMouseLeave={considerHidingTip}
								onFocusOut={handleFocusOff}
								ref={setTipEl}
								style={{
									position: pos.strategy,
									translate: `${pos.x}px ${pos.y}px`,
									visibility: visible() ? "visible" : "hidden",
									"--padding": `${padding()}px`,
								}}
								class="tooltip"
								classList={{
									animate: animate(),
									interactive: props.interactive,
								}}
							>
								<div class="base"></div>
								<div class="inner">{props.tip()}</div>
							</div>
						</Portal>
					</Show>
				);
			}, document.body);
		},
		showTip,
		considerHidingTip,
		setContentEl,
	};
}
