Introducing the 
What is a GAME ENGINE?
DESIGNING our GAME ENGINE
Project Setup 
Entry Point 
Logging 
Premake 
Planning the Event System
Event System
Precompiled Headers
Window Abstraction and GLFW
Window Events
Layers
Modern OpenGL (Glad)
ImGui
ImGui Events
Input Polling
Key and Mouse Codes
Maths
ImGui Docking and Viewports
Introduction to Rendering
Rendering Architecture
Rendering and Maintenance
Static Libraries and ZERO Warnings
Rendering Context
Our First Triangle!
OpenGL Shaders
Renderer API Abstraction
Vertex Buffer Layouts
Vertex Arrays
Renderer Flow and Submission
CAMERAS and How They Work
Creating an Orthographic Camera
Moving to Sandbox
TIMESTEPS and DELTA TIME
Transforms
Material Systems
Shader Abstraction and Uniforms
Refs, Scopes and Smart Pointers
TEXTURES
BLENDING
Shader Asset Files
Shader Library
How to Build a 2D Renderer
Camera Controllers
Resizing
Maintenance
Preparing for 2D Rendering
Starting our 2D Renderer
2D Renderer Transforms
2D Renderer Textures
Single Shader 2D Renderer
Intro to Profiling
Visual Profiling
Instrumentation
Improving our 2D Rendering API
How I Made a Game in an Hour Using Hazel
Hazel 2020
BATCH RENDERING
Batch Rendering Textures (+ Debugging!) 
Drawing Rotated Quads 
Renderer Stats and Batch Improvements 
Testing Hazel's Performance! 
Let's Make Something in Hazel! 
How Sprite Sheets/Texture Atlases Work 
SubTextures - Creating a Sprite Sheet API 
Creating a Map of Tiles 
Next Steps + Dockspace
Framebuffers
Making a New C++ Project in Hazel
Scene Viewport
Code Review + ImGui Layer Events
Where to go next + Code Review
Entity Component System
Intro to EnTT (ECS)
Entities and Components
The ENTITY Class
Camera Systems
Scene Camera
Native Scripting
Native Scripting (now with virtual functions!)
Scene Hierarchy Panel
Properties Panel
Camera Component UI
Drawing Component UI
Transform Component UI
Adding/Removing Entities and Components UI
Making the Hazelnut Editor Look Good!
Saving and Loading Scenes!
Open/Save File Dialogs
Transformation Gizmos
Editor Camera
Multiple Render Targets and Framebuffer Refactor 
Preparing Framebuffers for Mouse Picking 
Clearing Framebuffer Texture Attachments + Git Branching 
Mouse Picking 
Clicking to Select Entities 
SPIR-V and the New Shader System 
Content Browser/Asset Panel 
Content Browser Panel - ImGui Drag Drop 
Textures for Entities! 
Everything You Need in a 2D Game Engine (Hazel 2D) - Let's Talk
PLAY BUTTON! // Hazel 2D - Game Engine series
2D PHYSICS! 
Universally Unique Identifiers (UUID/GUID) 
Playing and Stopping Scenes (and Resetting) 
Rendering Circles in a Game Engine 
Rendering Lines in a Game Engine 
Circle Physics Colliders 
Visualizing Physics Colliders 


---------------------------------------------------------------------
メインループ マニアック解説〜これからのTime.deltaTime〜 - Unityステーション : https://www.youtube.com/watch?v=TP7N57r5Tqw&ab_channel=UnityJapan
より安定したゲームプレイを実現する Unity 2020.2 の Time.deltaTime 修正 ― それはどのように成されたのか https://blog.unity.com/ja/technology/fixing-time-deltatime-in-unity-2020-2-for-smoother-gameplay-what-did-it-take
※wgpuではvblankのタイミングが取得できないのとswapchainなどの複雑な機構はオミットされているので、パフォーマンス上必要になった場合は別のライブラリで対応
------------------------------------------------

mayaではマウスピッキングはイベントドリブン->renderスレッドは走らない
ただし、カットシーンではシミュレーションがあるかもしれないので、一応毎フレーム更新の方針で
