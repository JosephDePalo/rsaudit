local function is_bsd(conn)
	local re = regex.compile("NetBSD")
	local cmd_out = conn:run_cmd("uname -a")

	if re:is_match(cmd_out) then
		return "PASS", "SSH configuration is secure."
	else
		return "FAIL", "Not NetBSD"
	end
end

local function has_ufw(conn)
	local cmd_out = conn:run_cmd('dpkg-query -s ufw &>/dev/null && echo "good"')
	if string.find("good", cmd_out) then
		return "PASS", "ufw installed"
	else
		return "FAIL", "ufw not installed"
	end
end

-------------------------------------------------------------------
-- REGISTRATION
-------------------------------------------------------------------

register_check({
	id = "BSD-123",
	name = "Is NetBSD",
	description = "Checks if the target is NetBSD",
	severity = "Info",
	run = is_bsd,
})

register_check({
	id = "UBU-111",
	name = "ufw is Installed",
	description = "Checks if the ufw firewall is installed",
	severity = "Medium",
	run = has_ufw,
})
