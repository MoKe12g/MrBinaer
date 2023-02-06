#[derive(Copy, Clone, PartialEq)] // Arbeit an den Compiler weitergeben
// Every state, that is an animations,
// has to hold the start_frame as an integer32 to calculate the progress,
// the state has made at a given frame
pub enum SnowmanStates {
    Idle,
    // some times a snowman can be bored too
    Waving(i32),
    Jumping(i32),
    TakingTopHat(i32),
    HoldingTopHat(),
    PutTopHatBackOn(i32),
    // melting the snowman
    Melting(i32),
    Melted,
    ResurrectionInProgress(i32),
    // the snowman is able to become a bigger form of itself
    // the float is the targeted multiplier on the y-axis
    Shrinking(f32, i32),
    Growing(f32, i32),
    Big(f32),
    // special
    MorphingIntoAFirTree(i32),
    IsFirTree(),
    MorphingFromAFirTree(i32),
    // the snowman dont want to be clicked on, so it transforms itself in a way,
    // that it isn't anywhere near the unpleasant mouse pointer (who likes to be clicked on anyway?)
    DeformationToAvoidPoint(i32, i32, i32),
    IsDeformedToAvoidPoint(i32, i32),
    ReverseDeformationToAvoidPoint(i32, i32, i32),
}