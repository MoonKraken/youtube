const std = @import("std");
const httpz = @import("httpz");

const PORT = 8801;

const Person = struct { first_name: []u8, last_name: []u8, email: []u8 };

var stored_person: Person = undefined;
var stored_person_lock: std.Thread.RwLock = .{};

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn main() !void {
    const john: []u8 = try allocator.dupe(u8, "John");
    const smith: []u8 = try allocator.dupe(u8, "Smith");
    const email: []u8 = try allocator.dupe(u8, "jon@companyco.com");

    stored_person = Person{ .first_name = john, .last_name = smith, .email = email };

    var server = try httpz.Server(void).init(allocator, .{
        .port = PORT,
        .request = .{
            .max_form_count = 20,
        },
    }, {});

    defer server.deinit();
    defer server.stop();
    var router = server.router(.{});

    router.get("/", index, .{});
    router.get("/contact/1/edit", contact_edit, .{});
    router.put("/contact/1", contact_put, .{});
    router.get("/contact/1/", cancel_edit, .{});

    std.debug.print("listening http://localhost:{d}/\n", .{PORT});

    try server.listen();
}

fn view_only_person(a: std.mem.Allocator, person: *Person) ![]u8 {
    return try std.fmt.allocPrint(a,
        \\       <div hx-target="this" hx-swap="outerHTML">
        \\           <div><label>First Name</label>: {s}</div>
        \\           <div><label>Last Name</label>: {s}</div>
        \\           <div><label>Email</label>: {s}</div>
        \\           <button hx-get="/contact/1/edit" class="btn primary">
        \\           Click To Edit
        \\           </button>
        \\       </div>
    , .{ person.first_name, person.last_name, person.email });
}

fn index(_: *httpz.Request, res: *httpz.Response) !void {
    stored_person_lock.lockShared();
    defer stored_person_lock.unlockShared();
    res.body = try std.fmt.allocPrint(res.arena,
        \\           <!DOCTYPE html>
        \\           <html>
        \\           <head>
        \\               <title>+ HTMX</title>
        \\               <script src="https://unpkg.com/htmx.org"></script>
        \\               <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
        \\           </head>
        \\           <body>
        \\           {s}
        \\           </body>
        \\           </html>
    , .{try view_only_person(res.arena, &stored_person)});
}

fn contact_edit(_: *httpz.Request, res: *httpz.Response) !void {
    stored_person_lock.lockShared();
    defer stored_person_lock.unlockShared();
    res.body = try std.fmt.allocPrint(res.arena,
        \\<form hx-put="/contact/1" hx-target="this" hx-swap="outerHTML">
        \\<div>
        \\<label>First Name</label>
        \\<input type="text" name="first_name" value="{s}">
        \\</div>
        \\<div class="form-group">
        \\<label>Last Name</label>
        \\<input type="text" name="last_name" value="{s}">
        \\</div>
        \\<div class="form-group">
        \\<label>Email Address</label>
        \\<input type="email" name="email" value="{s}">
        \\</div>
        \\<button class="btn">Submit</button>
        \\<button class="btn" hx-get="/contact/1">Cancel</button>
        \\</form>
    , .{ stored_person.first_name, stored_person.last_name, stored_person.email });
}

fn cancel_edit(_: *httpz.Request, res: *httpz.Response) !void {
    stored_person_lock.lockShared();
    defer stored_person_lock.unlockShared();
    res.body = try view_only_person(res.arena, &stored_person);
}

fn contact_put(req: *httpz.Request, res: *httpz.Response) !void {
    stored_person_lock.lock();
    defer stored_person_lock.unlock();
    var it = (try req.formData()).iterator();

    res.content_type = .TEXT;

    while (it.next()) |kv| {
        if (std.mem.eql(u8, kv.key, "first_name")) {
            stored_person.first_name = try allocator.realloc(stored_person.first_name, kv.value.len);
            @memcpy(stored_person.first_name, kv.value);
        } else if (std.mem.eql(u8, kv.key, "last_name")) {
            stored_person.last_name = try allocator.realloc(stored_person.last_name, kv.value.len);
            @memcpy(stored_person.last_name, kv.value);
        } else if (std.mem.eql(u8, kv.key, "email")) {
            stored_person.email = try allocator.realloc(stored_person.email, kv.value.len);
            @memcpy(stored_person.email, kv.value);
        }
    }

    res.body = try view_only_person(res.arena, &stored_person);
}
