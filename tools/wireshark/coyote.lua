coyote_state_protocol = Proto("coyote.state",  "Coyote Device State Data")

do
    local statusValues = {
        [1] = "Inactive",
        [2] = "Connected",
        [3] = "Disconnected"
    }

    local f_load_status_a =
        ProtoField.uint8("coyote.state.load.a", "Load Status A", base.HEX, statusValues)

    local f_load_status_b =
        ProtoField.uint8("coyote.state.load.b", "Load Status B", base.HEX, statusValues)

    local t_unknown_1_a = ProtoField.uint8("coyote.state.unknown_1.a", "Unknown 1 A", base.HEX)
    local t_unknown_1_b = ProtoField.uint8("coyote.state.unknown_1.b", "Unknown 1 B", base.HEX)
    local t_unknown_2_a = ProtoField.uint8("coyote.state.unknown_2.a", "Unknown 2 A", base.HEX)
    local t_unknown_2_b = ProtoField.uint8("coyote.state.unknown_2.b", "Unknown 2 B", base.HEX)
    local t_unknown_3 = ProtoField.uint8("coyote.state.unknown_3", "Unknown 3", base.HEX)
    local t_unknown_4 = ProtoField.uint8("coyote.state.unknown_4", "Unknown 4", base.HEX)
    local t_unknown_5 = ProtoField.uint8("coyote.state.unknown_5", "Unknown 5", base.HEX)
    local t_unknown_6 = ProtoField.uint8("coyote.state.unknown_6", "Unknown 6", base.HEX)

    coyote_state_protocol.fields = {
        f_load_status_a,
        f_load_status_b,
        t_unknown_1_a,
        t_unknown_1_b,
        t_unknown_2_a,
        t_unknown_2_b,
        t_unknown_3,
        t_unknown_4,
        t_unknown_5,
        t_unknown_6
    }

    function coyote_state_protocol.dissector(buffer, pinfo, tree)
        length = buffer:len()
        if length ~= 10 then return end

        pinfo.cols.protocol = coyote_state_protocol.name

        local subtree = tree:add(coyote_state_protocol, buffer(), coyote_state_protocol.description)
        subtree:add_le(f_load_status_a, buffer(0, 1))
        subtree:add_le(f_load_status_b, buffer(1, 1))
        subtree:add_le(t_unknown_1_a, buffer(2, 1))
        subtree:add_le(t_unknown_1_b, buffer(3, 1))
        subtree:add_le(t_unknown_2_a, buffer(4, 1))
        subtree:add_le(t_unknown_2_b, buffer(5, 1))
        subtree:add_le(t_unknown_3, buffer(6, 1))
        subtree:add_le(t_unknown_4, buffer(7, 1))
        subtree:add_le(t_unknown_5, buffer(8, 1))
        subtree:add_le(t_unknown_6, buffer(9, 1))
    end
end

coyote_device_notification_protocol = Proto("coyote.notification", "Coyote Device Notification Data")

do
    local f_notification_type =
        ProtoField.uint8("coyote.notification.type", "Notification Type", base.HEX)

    coyote_device_notification_protocol.fields = {
        f_notification_type
    }

    function coyote_device_notification_protocol.dissector(buffer, pinfo, tree)
        length = buffer:len()

        pinfo.cols.protocol = coyote_device_notification_protocol.name
        
        local subtree = tree:add(coyote_device_notification_protocol, buffer(), coyote_device_notification_protocol.description)
        subtree:add_le(f_notification_type, buffer(0, 1))

        coyote_notification_dissector:try(buffer(0, 1):uint(), buffer(1, -1):tvb(), pinfo, tree)

    end
end

local function dissect_pawprint_payload(proto, buffer, pinfo, tree,
    f_pawprint_number, f_unknown_1, f_battery_level, f_unknown_2)
    local length = buffer:len()
    if length ~= 4 then return end
    pinfo.cols.protocol = proto.name
    local subtree = tree:add(proto, buffer(), proto.description)
    subtree:add_le(f_pawprint_number, buffer(0, 1))
    subtree:add_le(f_unknown_1,       buffer(1, 1))
    subtree:add_le(f_battery_level,   buffer(2, 1))
    subtree:add_le(f_unknown_2,       buffer(3, 1))
end

coyote_notification_02 = Proto("coyote.notification.02", "Pawprint Paired")

do
    local f_pawprint_number = ProtoField.uint8("coyote.notification.02.pawprint_number", "Pawprint Number", base.DEC)
    local f_unknown_1       = ProtoField.uint8("coyote.notification.02.unknown_1",       "Unknown 1",       base.HEX)
    local f_battery_level   = ProtoField.uint8("coyote.notification.02.battery_level",   "Battery Level",   base.DEC)
    local f_unknown_2       = ProtoField.uint8("coyote.notification.02.unknown_2",       "Unknown 2",       base.HEX)

    coyote_notification_02.fields = { f_pawprint_number, f_unknown_1, f_battery_level, f_unknown_2 }

    function coyote_notification_02.dissector(buffer, pinfo, tree)
        dissect_pawprint_payload(coyote_notification_02, buffer, pinfo, tree,
            f_pawprint_number, f_unknown_1, f_battery_level, f_unknown_2)
    end
end

coyote_notification_03 = Proto("coyote.notification.03", "No Pawprint Found")

do
    function coyote_notification_03.dissector(buffer, pinfo, tree)
        pinfo.cols.protocol = coyote_notification_03.name
        tree:add(coyote_notification_03, buffer(), coyote_notification_03.description)
    end
end

coyote_notification_0F = Proto("coyote.notification.0F", "Pawprint Connected")

do
    local f_pawprint_number = ProtoField.uint8("coyote.notification.0F.pawprint_number", "Pawprint Number", base.DEC)
    local f_unknown_1       = ProtoField.uint8("coyote.notification.0F.unknown_1",       "Unknown 1",       base.HEX)
    local f_battery_level   = ProtoField.uint8("coyote.notification.0F.battery_level",   "Battery Level",   base.DEC)
    local f_unknown_2       = ProtoField.uint8("coyote.notification.0F.unknown_2",       "Unknown 2",       base.HEX)

    coyote_notification_0F.fields = { f_pawprint_number, f_unknown_1, f_battery_level, f_unknown_2 }

    function coyote_notification_0F.dissector(buffer, pinfo, tree)
        dissect_pawprint_payload(coyote_notification_0F, buffer, pinfo, tree,
            f_pawprint_number, f_unknown_1, f_battery_level, f_unknown_2)
    end
end

coyote_notification_31 = Proto("coyote.notification.31", "Trigger Condition Set")

do
    local f_data = ProtoField.bytes("coyote.notification.31.data", "Trigger Condition Data")
    coyote_notification_31.fields = { f_data }

    function coyote_notification_31.dissector(buffer, pinfo, tree)
        pinfo.cols.protocol = coyote_notification_31.name
        local subtree = tree:add(coyote_notification_31, buffer(), coyote_notification_31.description)
        if buffer:len() > 0 then
            subtree:add(f_data, buffer(0, -1))
        end
    end
end

coyote_notification_51 = Proto("coyote.notification.51", "Unknown (Battery Echo?)")

do
    local f_unknown_1     = ProtoField.uint8("coyote.notification.51.unknown_1",     "Unknown 1",   base.HEX)
    local f_unknown_2     = ProtoField.uint8("coyote.notification.51.unknown_2",     "Unknown 2",   base.HEX)
    local f_battery_level = ProtoField.uint8("coyote.notification.51.battery_level", "Battery Level (echo)", base.DEC)

    coyote_notification_51.fields = { f_unknown_1, f_unknown_2, f_battery_level }

    function coyote_notification_51.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 3 then return end
        pinfo.cols.protocol = coyote_notification_51.name
        local subtree = tree:add(coyote_notification_51, buffer(), coyote_notification_51.description)
        subtree:add_le(f_unknown_1,     buffer(0, 1))
        subtree:add_le(f_unknown_2,     buffer(1, 1))
        subtree:add_le(f_battery_level, buffer(2, 1))
    end
end

coyote_notification_53 = Proto("coyote.notification.53", "Device Info")

do
    local f_reserved = ProtoField.uint8("coyote.notification.53.reserved", "Reserved", base.HEX)
    local f_bt_mac   = ProtoField.bytes("coyote.notification.53.bt_mac",   "BT MAC (last 4 bytes)")

    coyote_notification_53.fields = { f_reserved, f_bt_mac }

    function coyote_notification_53.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 5 then return end
        pinfo.cols.protocol = coyote_notification_53.name
        local subtree = tree:add(coyote_notification_53, buffer(), coyote_notification_53.description)
        subtree:add_le(f_reserved, buffer(0, 1))
        subtree:add(f_bt_mac,      buffer(1, 4))
    end
end

coyote_notification_61 = Proto("coyote.notification.61", "Unknown (0x60 Echo)")

do
    local f_button_id = ProtoField.uint8("coyote.notification.61.button_id", "Button ID",  base.DEC)
    local f_unknown_1 = ProtoField.uint8("coyote.notification.61.unknown_1", "Unknown 1",  base.HEX)
    local f_color     = ProtoField.uint8("coyote.notification.61.color",     "Color/Param", base.HEX)

    coyote_notification_61.fields = { f_button_id, f_unknown_1, f_color }

    function coyote_notification_61.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length < 2 then return end
        pinfo.cols.protocol = coyote_notification_61.name
        local subtree = tree:add(coyote_notification_61, buffer(), coyote_notification_61.description)
        subtree:add_le(f_button_id, buffer(0, 1))
        subtree:add_le(f_unknown_1, buffer(1, 1))
        if length >= 3 then
            subtree:add_le(f_color, buffer(2, 1))
        end
    end
end

coyote_notification_AE = Proto("coyote.notification.AE", "Waveforms Restored")

do
    function coyote_notification_AE.dissector(buffer, pinfo, tree)
        pinfo.cols.protocol = coyote_notification_AE.name
        tree:add(coyote_notification_AE, buffer(), coyote_notification_AE.description)
    end
end

coyote_notification_B1 = Proto("coyote.notification.B1", "Intensity Changed")

do
    local f_serial_number      = ProtoField.uint8("coyote.notification.B1.serial_number",      "Serial Number",      base.DEC)
    local f_intensity_a        = ProtoField.uint8("coyote.notification.B1.intensity_a",        "Channel A Intensity", base.DEC)
    local f_intensity_b        = ProtoField.uint8("coyote.notification.B1.intensity_b",        "Channel B Intensity", base.DEC)

    coyote_notification_B1.fields = { f_serial_number, f_intensity_a, f_intensity_b }

    function coyote_notification_B1.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 3 then return end
        pinfo.cols.protocol = coyote_notification_B1.name
        local subtree = tree:add(coyote_notification_B1, buffer(), coyote_notification_B1.description)
        subtree:add_le(f_serial_number, buffer(0, 1))
        subtree:add_le(f_intensity_a,   buffer(1, 1))
        subtree:add_le(f_intensity_b,   buffer(2, 1))
    end
end

coyote_notification_BE = Proto("coyote.notification.BE", "Limits Changed")

do
    local f_limit_a            = ProtoField.uint8("coyote.notification.BE.limit_a",            "Channel A Intensity Limit",       base.DEC)
    local f_limit_b            = ProtoField.uint8("coyote.notification.BE.limit_b",            "Channel B Intensity Limit",       base.DEC)
    local f_freq_balance_a     = ProtoField.uint8("coyote.notification.BE.freq_balance_a",     "Channel A Frequency Balance",     base.DEC)
    local f_freq_balance_b     = ProtoField.uint8("coyote.notification.BE.freq_balance_b",     "Channel B Frequency Balance",     base.DEC)
    local f_intensity_balance_a = ProtoField.uint8("coyote.notification.BE.intensity_balance_a", "Channel A Intensity Balance",   base.DEC)
    local f_intensity_balance_b = ProtoField.uint8("coyote.notification.BE.intensity_balance_b", "Channel B Intensity Balance",   base.DEC)

    coyote_notification_BE.fields = {
        f_limit_a, f_limit_b,
        f_freq_balance_a, f_freq_balance_b,
        f_intensity_balance_a, f_intensity_balance_b
    }

    function coyote_notification_BE.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 6 then return end
        pinfo.cols.protocol = coyote_notification_BE.name
        local subtree = tree:add(coyote_notification_BE, buffer(), coyote_notification_BE.description)
        subtree:add_le(f_limit_a,             buffer(0, 1))
        subtree:add_le(f_limit_b,             buffer(1, 1))
        subtree:add_le(f_freq_balance_a,      buffer(2, 1))
        subtree:add_le(f_freq_balance_b,      buffer(3, 1))
        subtree:add_le(f_intensity_balance_a, buffer(4, 1))
        subtree:add_le(f_intensity_balance_b, buffer(5, 1))
    end
end

coyote_notification_E0 = Proto("coyote.notification.E0", "Error")

do
    local errorCodes = {
        [0x01] = "Command type not understood",
        [0x02] = "Message format incorrect",
        [0x03] = "Cannot execute during accessory gameplay",
        [0x04] = "Object does not exist (re-fetch gameplay config)",
        [0x05] = "Writing to flash failed",
        [0x06] = "Command queue is full",
        [0x07] = "Unknown error"
    }

    local f_command    = ProtoField.uint8("coyote.notification.E0.command",    "Command",    base.HEX)
    local f_error_code = ProtoField.uint8("coyote.notification.E0.error_code", "Error Code", base.HEX, errorCodes)

    coyote_notification_E0.fields = { f_command, f_error_code }

    function coyote_notification_E0.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 2 then return end
        pinfo.cols.protocol = coyote_notification_E0.name
        local subtree = tree:add(coyote_notification_E0, buffer(), coyote_notification_E0.description)
        subtree:add_le(f_command,    buffer(0, 1))
        subtree:add_le(f_error_code, buffer(1, 1))
    end
end

coyote_notification_F1 = Proto("coyote.notification.F1", "Unknown (Intensity Echo?)")

do
    local f_serial_number = ProtoField.uint8("coyote.notification.F1.serial_number", "Serial Number",      base.DEC)
    local f_intensity_a   = ProtoField.uint8("coyote.notification.F1.intensity_a",   "Channel A Intensity", base.DEC)
    local f_intensity_b   = ProtoField.uint8("coyote.notification.F1.intensity_b",   "Channel B Intensity", base.DEC)

    coyote_notification_F1.fields = { f_serial_number, f_intensity_a, f_intensity_b }

    function coyote_notification_F1.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 3 then return end
        pinfo.cols.protocol = coyote_notification_F1.name
        local subtree = tree:add(coyote_notification_F1, buffer(), coyote_notification_F1.description)
        subtree:add_le(f_serial_number, buffer(0, 1))
        subtree:add_le(f_intensity_a,   buffer(1, 1))
        subtree:add_le(f_intensity_b,   buffer(2, 1))
    end
end

coyote_notification_F2 = Proto("coyote.notification.F2", "Unknown 0xF2 (Trigger Action Config?)")

do
    local f_data = ProtoField.bytes("coyote.notification.F2.data", "Data")
    coyote_notification_F2.fields = { f_data }

    function coyote_notification_F2.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 19 then return end
        pinfo.cols.protocol = coyote_notification_F2.name
        local subtree = tree:add(coyote_notification_F2, buffer(), coyote_notification_F2.description)
        subtree:add(f_data, buffer(0, -1))
    end
end

coyote_notification_F3 = Proto("coyote.notification.F3", "Unknown 0xF3")

do
    local f_data = ProtoField.bytes("coyote.notification.F3.data", "Data")
    coyote_notification_F3.fields = { f_data }

    function coyote_notification_F3.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 19 then return end
        pinfo.cols.protocol = coyote_notification_F3.name
        local subtree = tree:add(coyote_notification_F3, buffer(), coyote_notification_F3.description)
        subtree:add(f_data, buffer(0, -1))
    end
end

coyote_notification_F4 = Proto("coyote.notification.F4", "Unknown 0xF4")

do
    local f_data = ProtoField.bytes("coyote.notification.F4.data", "Data")
    coyote_notification_F4.fields = { f_data }

    function coyote_notification_F4.dissector(buffer, pinfo, tree)
        local length = buffer:len()
        if length ~= 13 then return end
        pinfo.cols.protocol = coyote_notification_F4.name
        local subtree = tree:add(coyote_notification_F4, buffer(), coyote_notification_F4.description)
        subtree:add(f_data, buffer(0, -1))
    end
end




coyote_notification_dissector = DissectorTable.new("coyote.notification", "Coyote Device Notification ", ftypes.UINT8, base.HEX, coyote_device_notification_protocol)
coyote_notification_dissector:add(0x02, coyote_notification_02)
coyote_notification_dissector:add(0x03, coyote_notification_03)
-- coyote_notification_dissector:add(0x04, ...)
-- coyote_notification_dissector:add(0x09, ...)
-- coyote_notification_dissector:add(0x0C, ...)
-- coyote_notification_dissector:add(0x0D, ...)
-- coyote_notification_dissector:add(0x0E, ...)
coyote_notification_dissector:add(0x0F, coyote_notification_0F)
-- coyote_notification_dissector:add(0x10, ...)
-- coyote_notification_dissector:add(0x21, ...)
-- coyote_notification_dissector:add(0x29, ...)
coyote_notification_dissector:add(0x31, coyote_notification_31)
-- coyote_notification_dissector:add(0x41, ...)
coyote_notification_dissector:add(0x51, coyote_notification_51)
coyote_notification_dissector:add(0x53, coyote_notification_53)
coyote_notification_dissector:add(0x61, coyote_notification_61)
-- coyote_notification_dissector:add(0x70, ...)
-- coyote_notification_dissector:add(0x99, ...)
-- coyote_notification_dissector:add(0xA1, ...)
-- coyote_notification_dissector:add(0xA3, ...)
-- coyote_notification_dissector:add(0xA4, ...)
coyote_notification_dissector:add(0xAE, coyote_notification_AE)
coyote_notification_dissector:add(0xB1, coyote_notification_B1)
-- coyote_notification_dissector:add(0xBD, ...)
coyote_notification_dissector:add(0xBE, coyote_notification_BE)
-- coyote_notification_dissector:add(0xC9, ...)
-- coyote_notification_dissector:add(0xD1, ...)
coyote_notification_dissector:add(0xE0, coyote_notification_E0)
-- coyote_notification_dissector:add(0xE2, ...)
-- coyote_notification_dissector:add(0xED, ...)
coyote_notification_dissector:add(0xF1, coyote_notification_F1)
coyote_notification_dissector:add(0xF2, coyote_notification_F2)
coyote_notification_dissector:add(0xF3, coyote_notification_F3)
coyote_notification_dissector:add(0xF4, coyote_notification_F4)

local bt_dissector = DissectorTable.get("bluetooth.uuid")
bt_dissector:add("2a59", coyote_state_protocol)
bt_dissector:add("150b", coyote_device_notification_protocol)
