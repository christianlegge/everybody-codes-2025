use seq_macro::seq;

seq!(N in 01..=20 {
    pub mod day~N;
});
