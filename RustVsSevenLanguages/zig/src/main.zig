const std = @import("std");
const SplitIterator = std.mem.SplitIterator;
fn sum_nums(it: *SplitIterator(u8)) !i32 {
    var sum:i32= 0;
    while (it.next()) |string|{
        sum += try std.fmt.parseInt(i32, string, 10);
    }
    return sum;
}
pub fn main() !void {
    const stdin_file = std.io.getStdIn().reader();

    var buf: [10]u8 = undefined;
    while (true) {
        if (try stdin_file.readUntilDelimiterOrEof(buf[0..], '\n')) |user_input| {
            var splits = std.mem.split(u8, user_input, " ");
            var sum = try sum_nums(&splits);
            const stdout_file = std.io.getStdOut().writer();
            var bw = std.io.bufferedWriter(stdout_file);
            const stdout = bw.writer();

            try stdout.print("{}\n", .{sum});
            try bw.flush(); // don't forget to flush!
        }
    }
}
