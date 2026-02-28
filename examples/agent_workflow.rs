//! Pincher Agent Engine - Full Integration Example
//!
//! This example demonstrates a complete agent workflow with Decapod governance.
//!
//! Run with: DECAPOD_SESSION_PASSWORD=your_password cargo run --example agent_workflow

use pincher::decapod::{
    broker::{EventEmitter, EventType},
    capabilities::CapabilitiesManager,
    coordination::{CoordinationManager, AgentType},
    rpc::RpcClient,
    session::Session,
    todo::TodoManager,
    validate::Validator,
    workspace::WorkspaceManager,
    workunit::WorkUnitManager,
};
use pincher::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦞 Pincher Agent Engine - Starting...\n");

    // Step 1: Discover Decapod capabilities
    println!("[1] Discovering Decapod capabilities...");
    let caps_mgr = CapabilitiesManager::new();
    let capabilities = caps_mgr.discover_json().await?;
    println!("    Decapod version: {}", capabilities.version);
    println!("    Available commands: {}", capabilities.commands.len());
    println!("    RPC operations: {}", capabilities.rpc_operations.len());

    // Step 2: Acquire session
    println!("\n[2] Acquiring Decapod session...");
    let password = std::env::var("DECAPOD_SESSION_PASSWORD")
        .expect("DECAPOD_SESSION_PASSWORD must be set");
    
    let session = Session::acquire(&password).await?;
    println!("    Session ID: {}", session.session_id());
    println!("    Token: {}...", &session.token()[..20.min(session.token().len())]);

    // Step 3: Create RPC client with session
    let rpc = RpcClient::new().with_session(session.token());

    // Step 4: Initialize agent
    println!("\n[3] Initializing agent...");
    let init_response = rpc.agent_init("pincher-example").await?;
    println!("    Agent initialized: {:?}", init_response.allowed_next_ops);

    // Step 5: Resolve context
    println!("\n[4] Resolving governance context...");
    let context = rpc
        .context_resolve(vec!["core", "interfaces"])
        .await?;
    println!("    Context capsule: {:?}", context.context_capsule.map(|c| c.scope));

    // Step 6: Validate project
    println!("\n[5] Running validation gates...");
    let validator = Validator::new();
    let validation = validator.run().await?;
    println!("    Validation passed: {}", validation.passed);
    
    if !validation.errors.is_empty() {
        println!("    Errors: {:?}", validation.errors);
    }

    // Step 7: Ensure workspace
    println!("\n[6] Ensuring workspace...");
    let workspace_mgr = WorkspaceManager::new().with_session(session.token());
    let workspace = workspace_mgr.ensure(Some("pincher-example")).await?;
    println!("    Workspace: {} @ {}", workspace.name, workspace.branch);

    // Step 8: Create task
    println!("\n[7] Creating task...");
    let todo_mgr = TodoManager::new().with_session(session.token());
    let task = todo_mgr
        .add("Complete pincher integration example", Some("high"), None)
        .await?;
    println!("    Task created: {} - {}", task.id, task.content);

    // Step 9: Claim task
    println!("\n[8] Claiming task...");
    let claimed = todo_mgr.claim(&task.id).await?;
    println!("    Task claimed by: {:?}", claimed.owner);

    // Step 10: Initialize workunit
    println!("\n[9] Initializing workunit...");
    let workunit_mgr = WorkUnitManager::new().with_session(session.token());
    let workunit = workunit_mgr
        .init(&task.id, "example-intent-ref")
        .await?;
    println!("    WorkUnit created: {}", workunit.id);

    // Step 11: Emit events
    println!("\n[10] Emitting events...");
    let emitter = EventEmitter::new("pincher-example", "executor")
        .with_session(session.session_id());
    
    let event = emitter.task_created(&task.id, "Complete integration example");
    println!("    Event: {} - {}", event.id, serde_json::to_string(&event.event_type)?);

    // Step 12: Multi-agent coordination
    println!("\n[11] Setting up coordination...");
    let coord = CoordinationManager::new("pincher-coordinator", AgentType::Coordinator);
    let plan = coord.create_coordination_plan(vec![
        ("Research implementation details", "researcher"),
        ("Write core integration", "executor"),
        ("Review and validate", "reviewer"),
    ]);
    println!("    Coordination plan: {} sub-agents", plan.sub_agents.len());

    // Step 13: Check governance response
    println!("\n[12] Checking governance...");
    let gov_response = rpc.context_scope("integration validation", Some(5)).await?;
    
    if gov_response.is_blocked() {
        println!("    ⛔ Governance: Blocked");
    } else if gov_response.has_advisory() {
        println!("    ⚠️  Governance: Has Advisory");
    } else {
        println!("    ✅ Governance: Proceed");
    }

    // Step 14: Complete task
    println!("\n[13] Completing task...");
    let completed = todo_mgr.complete(&task.id, Some("Integration example complete")).await?;
    println!("    Task status: {:?}", completed.status);

    // Step 15: Stop agent
    println!("\n[14] Stopping agent...");
    let stop_event = emitter.agent_stopped(Some("workflow complete"));
    println!("    Event: {} - {}", stop_event.id, serde_json::to_string(&stop_event.event_type)?);

    println!("\n🦞 Pincher Agent Engine - Complete!");
    println!("    All governance gates passed successfully.");

    Ok(())
}
