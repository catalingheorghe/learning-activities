**Web Programming**

 - HTML, CSS, JavaScript
 - websites and increasingly mobile applications

**Internet**

 - interconnected routers
 - standards for communication (protocols)
   - IP, DNS, TCP

**HTTP**

 - world wide web = a service on top of the internet
 - commands - GET, POST ...
 - URL: https://prefix|host-name.domain-name.top-level-domain/file

``` 
GET / HTTP/1.1
Host: www.example.com
...

HTTP/1.1 200 OK
Content-Type: text/html
...

```

 - browser -> developer tools
   - network - shows requests and response. E.g.: 301 Redirect (http->https)
   - 70 requests when going to harvard.edu (images, scripts, etc)
   - HTTP headers - lines in request and response (e.g.: Location)
   
 - a lot of status codes - 304 Not Modified, 500 Internal Server Error etc
 
Command line tool - "connect to an URL" - curl

`curl -I http://safetyschool.org`
 - `-I` - shows headers
 - => moved permanently => yale.edu
 
More complicated request URLs - passing input to servers

```
GET /search?q=cats HTTP/1.1
Host: www.google.com
...

```

**HTML**

Markup language
 - tags (elements) and attributes

First line: document type declaration - DOCTYPE

```
<!DOCTYPE html>
<html lang="en"> 
    ... 
</html>`
```
 
open source web-server for easy serving: http-server

Main tags: head, body
Other tags:
 - paragraphs `<p>`
 - headings: `<h1>, ... <h6>`
 - lists
   - unordered `<ul><li>...</li><ul>`
   - ordered `<ol>`
 - table `<table><tr><td>...</td></tr>...</table>`
 - image: `<img alt="harvard university" src="harvard.jpg">`
 - link:
   - anchor: `Visit <a href="image.html" id="harvard">Harvard</a>`
     or `href="https://www.harvard.edu"`
 - style tag:
```
<head>
<style>
a
{
	color: ##ff0000;
}

#harvard
{
    color: ##0000ff;
}

a:hover
{
    ...
}
...
</style>
```
 - ids: `#` - particular element, `.` - class

Input (html forms)
 - input in form -> action get to an url with parameters
```
<form action="http://www.google.com/search" method="get">
	<input name="q" type="search">
	<input type="submit" value="Search">
</form>
```

 - note: POST "hides" the q=.., the parameters; puts the info in the data,
   not in the URL

 - note: HTML entities e.g.: #169 -> copyright symbol
 
**CSS**

 - additional language for HTML
 - rely on properties (basically key-value pairs)
 - apply those properties to different elements via selectors
 

 - embed into html page in head -> style tag
 - or as attirbute to any tag `<header style="font-size: large;">`
 - or link to it via `<link>` under head (common name styles.css)
 
 - "cascading" - if `text-align: center` to body tag, it will cascade

```
selector
{
	prop-name: prop-value;
}
```

 - selectors
   - type selectors head, p, a etc
   - class selectors - `e.g: .large`; plus in html class="large"
   - id selectors (#something)

**JavaScript**

 - C like syntax
 - declaring a variable, loosely typed: `let counter = 0;`

 - embedded in a `<script>` tag
 - or linked to a file with `<script src="..."></src>`

 - html attribute: `<form onsubmit="greet()l return false;">`
   - the return instructs not to actually submit the form data to the server
   - just run the greet() function (which must be defined)

```
function greet()
{
	let name = document.querySelector('#name')
	alert('hello, ' + name);
}
```

 - document represent the web page in the browser - DOM (doc object model)
 - it has a method that can select one or more elements in the tree
 
Web programming is very event based.
Keeping JS and HTML separate is a good design, so better to do:

 - use a `<script>` tag
 - and add modify the form node dynamically to add an event callback with 
   the desired function for the submit event
 - note that browsers still parse the code top to bottom, so the script part
   being at the top will not find any of the elements in the page
   - so add a callback to do the above when the DOM is loaded

```
function listen()
{
	document.querySelect('form').addEventListener('submit', greet)
}

document.addEventListener('DOMContentLoaded', listen)
	
```

 - developer tools -> Console - see messages from JS code

 - anonymous functions - used commonly, if functions only used once
 
```
function listen()
{
	document.querySelect('form').addEventListener('submit', greet)
}

document.addEventListener('DOMContentLoaded', function() {
	document.querySelect('form').addEventListener('submit', function() {
		let name = document.querySelector('#name')
		alert('hello, ' + name);
	});
});
	
```

 - events: clicking, dragging, mouse down, up, touching etc
 
 - can modity content of DOM nodes - "inner HTML"
 - can modify CSS style properties of a node

 - developer tools -> Elements - html elements ("inspect" to go to an element)

### Shorts

#### HTTP

HTTP request line: <method request-target http-version>
the second line also includes the host name

response: <http-version status>
success - 200
redirection - 301, 302
client error - 401, 403, 404
server error - 500

chrome -> developer tools -> network tab

#### HTML

 - not a programming language (variable, logic, functions)
 - markup language - using tags to change how content is presented
 - indentation doesn't matter (whitespace is extra data sent over the wire)
 - no syntax errors, but should be well formed; use an (online) html validator

head

 - not part of the content of the html page, of the body
 - "title" tag -> name in browser tab
 
body

 - content of the page

b, i, u - bold, italic, underlined text

p - break text into paragraphs (whitespaces are ignored)
h1..6 - header text

ul, ol - undordered/orderd lists; li - list items
 - ol start=6 -> start numbering from 6

table - tr a row; td a column (table data)

form - input -> a field in an HTML form
 - input name=x type=y
 - no text inside the tag -> self-closing tag `<input ... />`
 - text, password, checkbox, radio, submit (nothing happens by def; refresh)
 
div - arbitrary page division

a href=x - anchor hyperlink
 - can be an internal link, or a http(s) link
 
img src=x ...

!DOCTYPE html
 - specific to html5 to inform the browser (html5 has some particular tags)
 
`!-- comment -->`


#### CSS

General

 - also not a programming language
 - a style language - how html elements should be modified

Structure

style-sheet
 - selector
 - { (declaration) key: value; ... }

body {
 background-color: blue
}
 
fonts
 - web safe fonts predefined in CSS
 - font-size can be specified in multiple ways (including referring to previous size)
 
selectors
 - tags - body, table, p ...
 - id (html attributes) - `#uniqueid`
 - class (html attributes) - `.students`
 
```
#unique
{
	border: 4px dotted blue;
	opacity: 0.7;
}
```

Comments

 `/* .... */`
 
CSS and HTML pages

style-sheets
 - embedded in html - style elements (in page's head)
 - linked to with `<link>` -> css file

### JavaScript

General
 - programming language derived from C (mid 1990s, a bit after Python)
 - HTML, CSS< JavaScript - web programming
 - file: `.js` extension
 - runs client-side, not server-side
 
In HTML
 - in `<script>` tags directly
 - or with `<link>` tags -> JavaScript file
 
Basics
 - variable - no type; to have it local `var` specifier
 - if, else if, else, switch
 - while, do while, for
 - functions defined with `function` keyword
 - can be anonymous - no name
 - arrays - `[1, 2, 3, 4, 5]`; elements can be of different types

It can behave as an object-oriented programming language (properties, methods)

New loop type - redux - iterate over key-value pairs
 - python `for key in list: ...`
 - js: `for (var key in array) { ... }`
 - also: `var of object` - iterates of values - key is the value directly
  
Concatenating string
 - use `+` (note: parseInt(day) if you add as int) - loosely typed

Everything in JS is a special case of an object
 - arrays methods: pop, push, size, shift
     - map method allows to apply a function to elements of array
     - good point to use an anon function to be mapped
 - a var is an object with only one property

``` 
var nums = [1, 2, 3];
nums = nums.map(function(num) { return num * 2; });
```
Events
 - hovering over something, clicking, typing etc
 - for all of these -> hs has event handlers
 - html elements have definitions for event handlers - e.g.: `<button onclick=alertname(event)...`
 
```
function alertname(event)
{
	var trigger = event.srcElement;
	alert('clicked on ' + trigger.innerHTML);
}
```

"printf": console.log() -> developer tools

### DOM

The "document object" - organizes the content of the entire web page. Can be
changed via JS, as it is an object.

In developer tools, you can see the document object:

 - console.dir(document) -> organizes the page into a directory structure
    - children properties -> html -> ...
    
Important properties for DOM

 - innerHTML
 - nodeName
 - id
 - parentNode
 - childNodes
 - attributes
 - style
 
Methods

 - getElementById(id)
 - getElementsByTagName(tag)
 - appendChild(node)
 - removeChild(node)

jQuery - JS library that simplifies client-side scripting, including DOM
manipulation

 - less code, but looks weirder
 - `$('#colorDiv').css('background-color', 'green');`
 (the dollar sign says JQuery)

#### Resources

 - html reference: https://www.w3schools.com/tags/default.asp
 - css reference: https://www.w3schools.com/cssref/
 - javascript https://www.w3schools.com/js/default.asp
 - jquery https://api.jquery.com






