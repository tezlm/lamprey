import { For, Show } from "solid-js";
import type { RoomT } from "./types.ts";
import { useCtx } from "./context.ts";
import { getTimestampFromUUID } from "sdk";
import { A, useNavigate } from "@solidjs/router";
import { useApi } from "./api.tsx";
import { AvatarWithStatus, UserView } from "./User.tsx";
import { tooltip } from "./Tooltip.tsx";

export const RoomMembers = (props: { room: RoomT }) => {
	const api = useApi();
	const room_id = () => props.room.id;
	const members = api.room_members.list(room_id);

	return (
		<ul class="member-list" data-room-id={props.room.id}>
			<For each={members()?.items}>
				{(member) => {
					const user_id = () => member.user_id;
					const user = api.users.fetch(user_id);

					function name() {
						let name: string | undefined | null = null;
						if (member?.membership === "Join") name ??= member.override_name;

						name ??= user()?.name;
						return name;
					}

					return tooltip(
						{
							placement: "left-start",
						},
						<Show when={user()}>
							<UserView
								user={user()}
								room_member={member}
							/>
						</Show>,
						<li class="menu-user" data-user-id={member.user_id}>
							<AvatarWithStatus user={user()} />
							<span class="text">
								<span class="name">{name()}</span>
								<Show when={false}>
									<span class="status-message">asdf</span>
								</Show>
							</span>
						</li>,
					);
				}}
			</For>
		</ul>
	);
};

export const RoomHome = (props: { room: RoomT }) => {
	const ctx = useCtx();
	const api = useApi();
	const nav = useNavigate();
	const room_id = () => props.room.id;

	const threads = api.threads.list(room_id);

	function createThread(room_id: string) {
		ctx.dispatch({
			do: "modal.prompt",
			text: "name?",
			cont(name) {
				if (!name) return;
				ctx.client.http.POST("/api/v1/room/{room_id}/thread", {
					params: {
						path: { room_id },
					},
					body: { name },
				});
			},
		});
	}

	function leaveRoom(_room_id: string) {
		ctx.dispatch({
			do: "modal.confirm",
			text: "are you sure you want to leave?",
			cont(confirmed) {
				if (!confirmed) return;
				ctx.client.http.DELETE("/api/v1/room/{room_id}/member/{user_id}", {
					params: {
						path: {
							room_id: props.room.id,
							user_id: api.users.cache.get("@self")!.id,
						},
					},
				});
			},
		});
	}

	const n = useNavigate();
	// <div class="date"><Time ts={props.thread.baseEvent.originTs} /></div>
	return (
		<div class="room-home">
			<h2>{props.room.name}</h2>
			<p>{props.room.description}</p>
			<button onClick={() => createThread(room_id())}>create thread</button>
			<br />
			<button onClick={() => leaveRoom(room_id())}>leave room</button>
			<br />
			<br />
			<form
				onSubmit={async (e) => {
					e.preventDefault();
					const message = e.target.querySelector("input")?.value;
					if (!message) return;
					const t = await ctx.client.http.POST(
						"/api/v1/room/{room_id}/thread",
						{
							params: {
								path: { room_id: props.room.id },
							},
							body: { name: "thread" },
						},
					);
					if (!t.data) return;
					ctx.dispatch({
						do: "thread.send",
						thread_id: t.data.id,
						text: message,
					});
					n(`/thread/${t.data.id}`);
				}}
			>
				<label>
					quick create thread<br />
					<input type="text" placeholder="message" autofocus />
				</label>
			</form>
			<br />
			<A href={`/room/${props.room.id}/settings`}>settings</A>
			<br />
			<ul>
				<For
					each={[
						...threads()?.items.filter((i) =>
							i.room_id === props.room.id && i.state !== "Deleted"
						) ??
							[],
					]}
				>
					{(thread) => (
						<li>
							<article class="thread">
								<header onClick={() => nav(`/thread/${thread.id}`)}>
									<div class="top">
										<div class="icon"></div>
										<div class="spacer">{thread.name}</div>
										<div class="time">
											Created at{" "}
											{getTimestampFromUUID(thread.id).toDateString()}
										</div>
									</div>
									<div
										class="bottom"
										onClick={() => nav(`/thread/${thread.id}`)}
									>
										{thread.message_count} messages &bull; last msg{" "}
										{getTimestampFromUUID(thread.last_version_id ?? thread.id)
											.toDateString()}
										<Show when={thread.description}>
											<br />
											{thread.description}
										</Show>
									</div>
								</header>
								<Show when={true}>
									<div class="preview">
										<For each={[]}>
											{(_msg) => "todo: show message here?"}
										</For>
										<details>
											<summary>json data</summary>
											<pre>
												{JSON.stringify(thread, null, 4)}
											</pre>
										</details>
									</div>
								</Show>
								<Show when={false}>
									<footer>message.remaining</footer>
								</Show>
							</article>
						</li>
					)}
				</For>
			</ul>
		</div>
	);
};
