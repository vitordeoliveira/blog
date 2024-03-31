---
filename: "what_are_axum_askama_htmx_and_why_use_it"
title: "What are Axum + Askama + HTMX and Why use it?"
subtitle: "A quick explanation about Axum, Askama and HTMX"
description: "I am a fan of some tools and technologies, I think all my blog posts make this clear to everyone.
Today, I will show to you guys what and why to use those tecnologies on Rust.
"
tags: ["rust", "axum", "htmx", "askama"]
similar_posts: ["why_js_devs_are_migrating_from_js_to_rust"]
date: "2024-03-31t17:52:00"
finished: true
---

# What are Axum + Askama + HTMX and Why use it?

I am a fan of some tools and technologies, I think all my blog posts make this clear to everyone.

Today, I will show to you guys [what](#what-is-htmx) and [why](#why-use-these-tools) to use those tecnologies on Rust.

> if you already know what axum, askama and htmx is and why you should use them, just [click here](), and we jump to HOW blog post.

OK, now lets go to **What** is the tools, they say "start with the why", but I dont saw the reason to explain the why, if you not even know what the tools are haha

So lets go to the definitions:
- [What is HTMX?](#what-is-htmx)
- [What is Askama?](#what-is-askama)
- [What is Axum?](#what-is-axum)

## <a name="what-is-htmx"></a> What is HTMX?
HTMX is a nutshell HTML as language :D

It is a JS script that gives super powers to html, different from React and others that is a JS script that generates html.

The idea is that HTML can from EVERY tag dispatch an event to the backend, and deal with the response (that should also be html).

In raw html only few tags can dispatch an event to backend, the \<a\> tag and the form with submit.

HTMX allow every element to do that...

This is very simple, elegant and powerful, because this way you just use javascript where is needed and not in everything how is done in the modern frameworks.

that also allow the backend devs to use the language of choice in the frontend, making you run away from JS or TS.

## <a name="what-is-askama"></a> What is Askama
Askama is simple the rust crate that we will use to make Rust read and manipulate HTML.
## <a name="what-is-axum"></a> What is Axum?
Axum is a Rust crate that allow us to serve a web application framework.

So in a nutshell he is the one we use to create the server, routes and etc...

## <a name="why-use-these-tools"></a> Why use these tools?

HTMX to simplify the frontend...

We DONT NEED a heavy and very confusing SPA to write a simple blog... to be honest more I code on those tools more I think we not even need SPAs for nothing...

Ok, lets put this in different words, the SPAs was created to have more dynamic pages, and bring a better user experience, they have their value. But in a word with blazzing fast internet, and a awesome infrastrucure tools, MAYBE and just maybe, we should have the backend deal with the state of the application, and rely in the Frontend to be just a view, and not a copy of the state of the backend.

What I mean with that? With SPAs the frontend is a JS bundle that is downloaded in the user browser, and generate the html in the client-side, after this the client makes a request to backend requesting the data to be loaded, so the backend sent to client a JSON, who is on this point in time a copy of the state of backend, then the client takes this JSON and react (Now you understand why the ReactJs has this name :D ) to it, updating the client on runtime. 

Here is a very simplified draw of this explanation: 

![[Bildschirmfoto vom 2024-03-31 14-43-08.png]]

And here is a very simplified draw of what we want to achieve with HTMX

![[Bildschirmfoto vom 2024-03-31 16-42-22.png]]

As you can see on this second draw we just have ONE state of the application, this way the view/Frontend don't need to store information, the client shouldn't know anything about the logic of the application.

Another advantage of HTMX is the fact, that now you can create the entire application using whatever language you like, here we are using Rust, but you can do with, GO, Java, Rails and even NodeJS... as the frontend is not forced to be only JS anymore, you can use the backend of your preference.

Ok, but that is exacly how it was in past, so we are just backing to old days right?

Kinda... the difference here is that with HTMX you dont need to render the entire page, the backend will just send a HTML fragment, and the HTMX in the client side will take care of where this new HTML should be placed... This changes some stuff, like in the past when we navigate to another page we have a "blink" of a half of a second because the entire page is been updated, with HTMX we dont have this, because we are just swaping or appending divs in the HTML of the client side.

If you want to understand better, how HTMX works, I recommend to watch and read those links:

Official page:

<a href="https://htmx.org/" target="_blank">https://htmx.org/</a>

The PrimeGOage video on Frontend Masters


<a href="https://www.youtube.com/watch?v=SZ0nR3QHebM" target="_blank">
Youtube PrimeGOage Frontend Masters
</a>

## How implement then in RUST?

The real and fun part is a different blog post, just [click here]() to go there :D




