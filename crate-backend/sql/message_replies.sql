with recursive message_tree as (
    select
        id,
        reply_id,
        1 as depth
    from
        message
    where
        id = $2
    union all
    select
        m.id,
        m.reply_id,
        mt.depth + 1
    from
        message m
        join message_tree mt on m.reply_id = mt.id
    where
        mt.depth < $3
),
ranked_messages as (
    select
        id,
        reply_id,
        row_number() over (partition by reply_id order by id) as rn
    from
        message_tree
),
message_reaction as (
    -- select message_id, key, count(*), bool_or(user_id = $123) as self
    select
        message_id,
        json_agg((select row_to_json(j) from (select key, count(*) as count) j)) as json
    from reaction
    group by message_id
    order by min(position)
)
select
    msg.type as "message_type: DbMessageType",
    msg.id,
    msg.thread_id, 
    msg.version_id,
    msg.ordering,
    msg.content,
    msg.metadata,
    msg.reply_id,
    msg.override_name,
    msg.author_id,
    msg.edited_at,
    msg.deleted_at,
    coalesce(att_json.attachments, '{}') as "attachments!",
    msg.embeds as "embeds",
    r.json as "reactions"
from message as msg
join ranked_messages rm on msg.id = rm.id
left join att_json on att_json.version_id = msg.version_id
left join message_reaction r on r.message_id = msg.id
where is_latest and thread_id = $1 and msg.deleted_at is null and (rm.rn <= $4 or $4 is null)
  and msg.id > $5 AND msg.id < $6
order by (CASE WHEN $7 = 'f' THEN msg.id END), msg.id DESC LIMIT $8
