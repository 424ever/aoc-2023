const std = @import("std");
const expect = std.testing.expect;

pub fn part1(reader: anytype, alloc: std.mem.Allocator) !u32 {
    var numbers = std.ArrayList(u16).init(alloc);
    defer numbers.deinit();
    var buffer: [1024]u8 = undefined;

    while (try reader.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        var first_number: ?u8 = null;
        var last_number: ?u8 = null;

        for (line) |char| {
            if (char >= '0' and
                char <= '9')
            {
                if (first_number == null) {
                    first_number = char - '0';
                }
                last_number = char - '0';
            }
        }
        try numbers.append(first_number.? * 10 + last_number.?);
    }
    var sum: u32 = 0;
    for (numbers.items) |num| {
        sum += num;
    }
    return sum;
}

fn numberHere(ptr: []const u8, words: std.StringHashMap(u8)) ?u8 {
    if (ptr[0] >= '0' and ptr[0] <= '9')
        return ptr[0] - '0';

    var iterator = words.iterator();

    while (iterator.next()) |entry| {
        if (std.mem.startsWith(u8, ptr, entry.key_ptr.*)) {
            return entry.value_ptr.*;
        }
    }

    return null;
}

pub fn part2(reader: anytype, alloc: std.mem.Allocator) anyerror!u32 {
    var numbers = std.ArrayList(u16).init(alloc);
    defer numbers.deinit();
    var buffer: [1024]u8 = undefined;

    var words = std.StringHashMap(u8).init(alloc);
    defer words.deinit();

    try words.put("one", 1);
    try words.put("two", 2);
    try words.put("three", 3);
    try words.put("four", 4);
    try words.put("five", 5);
    try words.put("six", 6);
    try words.put("seven", 7);
    try words.put("eight", 8);
    try words.put("nine", 9);

    while (try reader.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        var first_number: ?u8 = null;
        var last_number: ?u8 = null;

        for (0..line.len) |i| {
            const num = numberHere(line[i..], words);
            if (num != null) {
                if (first_number == null)
                    first_number = num.?;
                last_number = num.?;
            }
        }

        try numbers.append(first_number.? * 10 + last_number.?);
    }
    var sum: u32 = 0;
    for (numbers.items) |num| {
        sum += num;
    }
    return sum;
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var args = std.process.args();
    _ = args.skip();
    const file_path = args.next().?;
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();
    var bufreader = std.io.bufferedReader(file.reader());
    const reader = bufreader.reader();
    try std.io.getStdOut().writer().print("{}\n", .{try part1(reader, allocator)});
    try file.seekTo(0);
    try std.io.getStdOut().writer().print("{}\n", .{try part2(reader, allocator)});
}

test "part 1" {
    const lines =
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    var stream = std.io.fixedBufferStream(lines);
    const reader = stream.reader();
    const result: u16 = try part1(reader, std.testing.allocator);
    try expect(result == 142);
}

test "part 2" {
    const lines =
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;
    var stream = std.io.fixedBufferStream(lines);
    const reader = stream.reader();
    const result = try part2(reader, std.testing.allocator);
    try expect(result == 281);
}
