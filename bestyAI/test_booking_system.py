import pytest
import asyncio
from property_python import BookingSystem

@pytest.fixture
def booking_system():
    system = BookingSystem()
    system.initialize()
    yield system
    system.clear()

@pytest.mark.asyncio
async def test_single_booking(booking_system):
    result = await booking_system.schedule_maintenance_request(
        property_id="P1",
        time_slot="2024-07-03T10:00",
        technician_id="T1",
        booking_data={"description": "Test booking"}
    )
    assert result["success"], "Single booking should succeed"

@pytest.mark.asyncio
async def test_sequential_double_booking_property(booking_system):
    # First booking should succeed
    result1 = await booking_system.schedule_maintenance_request(
        property_id="P2",
        time_slot="2024-07-03T11:00",
        technician_id="T2",
        booking_data={"description": "First booking"}
    )
    # Second booking for same property/time should fail
    result2 = await booking_system.schedule_maintenance_request(
        property_id="P2",
        time_slot="2024-07-03T11:00",
        technician_id="T3",
        booking_data={"description": "Second booking"}
    )
    assert result1["success"]
    assert not result2["success"]
    assert result2["reason"] == "Property not available"

@pytest.mark.asyncio
async def test_sequential_double_booking_technician(booking_system):
    # First booking should succeed
    result1 = await booking_system.schedule_maintenance_request(
        property_id="P3",
        time_slot="2024-07-03T12:00",
        technician_id="T4",
        booking_data={"description": "First booking"}
    )
    # Second booking for same technician/time should fail
    result2 = await booking_system.schedule_maintenance_request(
        property_id="P4",
        time_slot="2024-07-03T12:00",
        technician_id="T4",
        booking_data={"description": "Second booking"}
    )
    assert result1["success"]
    assert not result2["success"]
    assert result2["reason"] == "Technician not available"

@pytest.mark.asyncio
async def test_cancellation_and_rebooking(booking_system):
    # Book and then cancel
    result = await booking_system.schedule_maintenance_request(
        property_id="P5",
        time_slot="2024-07-03T13:00",
        technician_id="T5",
        booking_data={"description": "Cancelable booking"}
    )
    assert result["success"]
    booking_id = result["booking_id"]
    cancel_result = booking_system.cancel_booking(booking_id)
    assert cancel_result["success"]

    # Now rebook same slot, should succeed
    result2 = await booking_system.schedule_maintenance_request(
        property_id="P5",
        time_slot="2024-07-03T13:00",
        technician_id="T5",
        booking_data={"description": "Rebooking"}
    )
    assert result2["success"]

@pytest.mark.asyncio
async def test_concurrent_property_booking(booking_system):
    # Try to book the same property/time with multiple technicians concurrently
    async def try_booking(tech_id):
        return await booking_system.schedule_maintenance_request(
            property_id="P6",
            time_slot="2024-07-03T14:00",
            technician_id=tech_id,
            booking_data={"description": f"Concurrent booking by {tech_id}"}
        )
    results = await asyncio.gather(
        try_booking("T6a"),
        try_booking("T6b"),
        try_booking("T6c"),
    )
    successes = [r for r in results if r["success"]]
    assert len(successes) == 1, f"Only 1 booking should succeed per property/time slot, got {len(successes)}"

@pytest.mark.asyncio
async def test_concurrent_technician_booking(booking_system):
    # Try to book the same technician/time with multiple properties concurrently
    async def try_booking(prop_id):
        return await booking_system.schedule_maintenance_request(
            property_id=prop_id,
            time_slot="2024-07-03T15:00",
            technician_id="T7",
            booking_data={"description": f"Concurrent booking for T7 by {prop_id}"}
        )
    results = await asyncio.gather(
        try_booking("P7a"),
        try_booking("P7b"),
        try_booking("P7c"),
    )
    successes = [r for r in results if r["success"]]
    assert len(successes) == 1, f"Only 1 booking should succeed per technician/time slot, got {len(successes)}"
