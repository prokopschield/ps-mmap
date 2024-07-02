use crate::*;

#[test]
pub fn drop() -> Result<()> {
    let mapping = MemoryMapping::new_blank(MmapOptions::new().len(1000))?;
    let mapping2 = mapping.clone();
    let mapping3 = mapping2.rw()?;

    {
        let mut guard = mapping3.lock()?;

        guard[15] = 2;
    }

    let mapping4 = mapping3.done();
    let mapping5 = mapping.clone().rw()?;

    {
        let mut guard = mapping5.lock()?;

        guard[15] += 1;
    }

    assert_eq!(mapping4[15], 3);

    Ok(())
}
