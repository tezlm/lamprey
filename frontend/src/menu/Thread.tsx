import { useNavigate } from "@solidjs/router";
import { useApi } from "../api.tsx";
import { useCtx } from "../context.ts";
import { Item, Menu, Separator, Submenu } from "./Parts.tsx";
import { Match, Switch } from "solid-js";

// the context menu for threads
export function ThreadMenu(props: { thread_id: string }) {
	const ctx = useCtx();
	const api = useApi();
	const nav = useNavigate();

	const thread = api.threads.fetch(() => props.thread_id);
	const copyId = () => navigator.clipboard.writeText(props.thread_id);
	const markRead = () => {
		const thread = api.threads.cache.get(props.thread_id)!;
		const version_id = thread.last_version_id;
		ctx.dispatch({
			do: "thread.mark_read",
			thread_id: props.thread_id,
			also_local: true,
			version_id,
		});
	};

	const deleteThread = () => {
		ctx.dispatch({
			do: "modal.confirm",
			text: "are you sure you want to leave?",
			cont(confirmed) {
				if (!confirmed) return;
				ctx.client.http.DELETE("/api/v1/thread/{thread_id}", {
					params: {
						path: { thread_id: props.thread_id },
					},
				});
			},
		});
	};

	const copyLink = () => {
		const url = `${ctx.client.opts.apiUrl}/thread/${props.thread_id}`;
		navigator.clipboard.writeText(url);
	};

	const logToConsole = () => console.log(JSON.parse(JSON.stringify(thread())));

	const settings = (to: string) => () =>
		nav(`/thread/${props.thread_id}/settings${to}`);

	const closeThread = () => {
		ctx.client.http.PUT("/api/v1/thread/{thread_id}/archive", {
			params: { path: { thread_id: props.thread_id } },
		});
	};

	const openThread = () => {
		ctx.client.http.PUT("/api/v1/thread/{thread_id}/activate", {
			params: { path: { thread_id: props.thread_id } },
		});
	};

	return (
		<Menu>
			<Item onClick={markRead}>mark as read</Item>
			<Item onClick={copyLink}>copy link</Item>
			<ThreadNotificationMenu />
			<Separator />
			<Submenu content={"edit"} onClick={settings("")}>
				<Item onClick={settings("")}>info</Item>
				<Item>permissions</Item>
				<Submenu content={"tags"}>
					<Item>foo</Item>
					<Item>bar</Item>
					<Item>baz</Item>
				</Submenu>
			</Submenu>
			<Item>pin</Item>
			<Switch>
				<Match when={thread()?.state === "Active"}>
					<Item onClick={closeThread}>close</Item>
				</Match>
				<Match when={thread()?.state === "Archived"}>
					<Item onClick={openThread}>open</Item>
				</Match>
			</Switch>
			<Item>lock</Item>
			<Item onClick={deleteThread}>delete</Item>
			<Separator />
			<Item onClick={copyId}>copy id</Item>
			<Item onClick={logToConsole}>log to console</Item>
		</Menu>
	);
}

function ThreadNotificationMenu() {
	return (
		<>
			<Submenu content={"notifications"}>
				<Item>
					<div>default</div>
					<div class="subtext">
						Uses the room's default notification setting.
					</div>
				</Item>
				<Item>
					<div>everything</div>
					<div class="subtext">
						You will be notified of all new messages in this thread.
					</div>
				</Item>
				<Item>
					<div>watching</div>
					<div class="subtext">
						Messages in this thread will show up in your inbox.
					</div>
				</Item>
				<Item>
					<div>mentions</div>
					<div class="subtext">You will only be notified on @mention</div>
				</Item>
				<Separator />
				<Item>bookmark</Item>
				<Submenu content={"remind me"}>
					<Item>in 15 minutes</Item>
					<Item>in 3 hours</Item>
					<Item>in 8 hours</Item>
					<Item>in 1 day</Item>
					<Item>in 1 week</Item>
				</Submenu>
			</Submenu>
			<Submenu content={"mute"}>
				<Item>for 15 minutes</Item>
				<Item>for 3 hours</Item>
				<Item>for 8 hours</Item>
				<Item>for 1 day</Item>
				<Item>for 1 week</Item>
				<Item>forever</Item>
			</Submenu>
		</>
	);
}
