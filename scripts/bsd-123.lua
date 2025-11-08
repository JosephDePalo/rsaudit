---
-- id: BSD-123
-- name: Is NetBSD
-- description: Checks if the target is NetBSD
-- severity: Info
---

METADATA = {
	id = "BSD-123",
	name = "Is NetBSD",
	description = "Checks if the target is netBSD",
	severity = "Info",
}

function run_check()
	local re = regex.compile("NetBSD")
	local cmd_out = conn:run_cmd("uname -a")

	if re:is_match(cmd_out) then
		return { status = "Pass", details = "SSH configuration is secure." }
	else
		return { status = "Fail", details = "Not NetBSD" }
	end
end
