== counts ==
[[example]]=3
sddgate=9
event_stream=102
== readme 21具对表 ==
187:写十二个（外部分级十 + 本地可信位二），外部分级十个全部要求先 `lease_open` 立会话，未立会话一律拒写：`lease_open`、`record_intent`、`record_append`、`record_park`、`record_direct`（本地可信位直写）、`lease_lock`、`lease_unlock`、`lease_wait_turn`、`lease_claim`、`lease_unclaim`（本地可信位）、`lease_commit`、`lease_close`。三个 record 是三类留痕：意图、事件、停泊。逐件契约见 doc/spec/SPEC-023。
