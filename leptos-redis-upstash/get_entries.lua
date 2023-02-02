local entries_ss = ARGV[1] .. ':entries'
local start_arg = '[' .. ARGV[2]
local end_arg = '(' .. ARGV[3]
local entry_dates = redis.call('ZRANGE', entries_ss, start_arg, end_arg, 'bylex')
local response = {}
for n=1, #entry_dates, 1 do
  local date = entry_dates[n]
  table.insert(response, date)
  table.insert(response, redis.call('GET', ARGV[1] .. ':' .. date))
end
return response
