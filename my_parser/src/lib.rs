//  .       어떤 문자 1개
//  []      안에 있는 문자 중 1개
//  [^ ]    안에 있는 문자를 제외한 문자 중 1개
//  *       바로 앞의 문자가 0 또는 그 이상 반복
//  +       바로 앞의 문자가 1 또는 그 이상 반복
//  ?       바로 앞의 문자가 0 또는 1번 반복
//  {n}     바로 앞의 문자가 n번 반복
//  {n,m}   바로 앞의 문자가 n번 이상 m번 이하 반복
//  ^       줄의 제일 처음 (시작 위치)
//  $       줄의 제일 마지막 (끝 위치)
//  ()      Group, 특정 패턴을 묶어서 반복기호 등과 함께 사용
//  (|)     그룹 내에 | 로 나눠진 패턴 중 하나와 일치하는 것
//  ()\숫자  그룹으로 지정된 내용을 \숫자의 형식으로 재 사용하는 것
//  Greedy, Lazy
//  반복 문자 개수 탐지
//  가능한 최대의 것 Greedy,    <.*>    *   +   ?   {n}    {n,}    {n,m}
//  가능한 최소의 것 Lazy,      <.*?>   *?  +?  ??  {n}?   {n,}?   {n,m}?
//  주요 Escape 문자
//  \t  \r  \n  \s [ \t\n\r\f\v]  \S [^ \t\n\r\f\v]  \d [0-9]  \D [^0-9]  \w [a-zA-Z0-9_]  \W [^a-zA-Z0-9_]
//  \b  \\  \u0020  \x20
//  203.255.241.165 - - [31/Oct/2011:19:48:43 +0900] "GET /images/company/his_2007.gif HTTP/1.1" 304 0 "http://www.infnis.com/company/history.htm" "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0; SLCC2; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30729; Media Center PC 6.0; InfoPath.3; Tablet PC 2.0; .NET4.0C)"
//  (?P<ip>\S*)[^\[]*(?P<timedate>\[[^\]]*\])\s(?P<method_string>"(?P<method>[A-Z]+)\s(?P<mtd>[^"]*)")\s(?P<num>[0-9]{3}\s0)\s(?P<url>"([^"]*"))\s(?P<info>"([^"]*"))\n?
pub mod parse;
pub mod regular;
