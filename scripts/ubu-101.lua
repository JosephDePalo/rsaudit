---
-- id: UBU-101
-- name: ufw Installed
-- description: Checks if the ufw firewall is installed.
-- severity: Low
---

METADATA = {
	id = "UBU-101",
	name = "ufw Installed",
	description = "Checks if the ufw firewall is installed",
	severity = "Low",
}

function run_check()
	local cmd_out = conn:run_cmd('dpkg-query -s ufw &>/dev/null && echo "good"')
	if string.find(cmd_out, "good") then
		return { status = "Pass", details = "UFW is installed." }
	else
		return { status = "Fail", details = "UFW is not installed." }
	end
end
