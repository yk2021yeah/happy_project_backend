# HappyProject backend

## Concept

This project aims to create an project management web application which is simple and everyone can use it as a single source of project related items. From task scheduling to issue tracking, every item will need user-defined level of approval. To eliminate over-committed milestones, unhandled development schedule, etc... Motivated to achieve elimination of any spread sheet application to manage projects. Bunch of tabbed sheets, uncotrolled copies spread out to unknown folks.

## This backend

Utilizing MongoDB Atlas free account as database and the backend web server API is written in Rust. As a server, axum was picked up since it's under Tokio. At first, preparing CRUD API of project data (note: current code is just at starting point and using non-related sample data provided by MongoDB free account.)

## As an API server to interact with MongoDB

Aim to be a boilerplate project to build REST API server to Mongo. From basic CRUD operations to advanced topics will be implemented.

## Frontend

will be created as Webassembly by yew stack.
