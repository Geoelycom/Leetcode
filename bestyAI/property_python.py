import asyncio
import random
import time
from typing import Dict, Optional, Any
from datetime import datetime
from collections import defaultdict


class MockDatabase:
    def __init__(self):
        self.bookings: Dict[str, Dict[str, Any]] = {}
        self.technician_bookings: Dict[str, Dict[str, Any]] = {}
        self.connections = 0

    def initialize(self):
        self.bookings.clear()
        self.technician_bookings.clear()
        self.connections = 0

    def clear(self):
        self.bookings.clear()
        self.technician_bookings.clear()
        self.connections = 0

    async def check_availability(self, property_id: str, time_slot: str) -> bool:
        self.connections += 1
        await asyncio.sleep(random.uniform(0.01, 0.05))
        key = f"{property_id}-{time_slot}"
        is_available = key not in self.bookings
        self.connections -= 1
        return is_available

    async def check_technician_availability(self, technician_id: str, time_slot: str) -> bool:
        self.connections += 1
        await asyncio.sleep(random.uniform(0.01, 0.05))
        key = f"{technician_id}-{time_slot}"
        is_available = key not in self.technician_bookings
        self.connections -= 1
        return is_available

    async def create_booking(self, property_id: str, time_slot: str, technician_id: str, booking_data: Dict[str, Any]) -> Dict[str, Any]:
        self.connections += 1
        await asyncio.sleep(random.uniform(0.02, 0.08))
        key = f"{property_id}-{time_slot}"
        self.bookings[key] = {
            'property_id': property_id,
            'time_slot': time_slot,
            'technician_id': technician_id,
            **booking_data,
            'created_at': datetime.now().isoformat()
        }
        self.connections -= 1
        return {'success': True, 'booking_id': booking_data['booking_id']}

    async def reserve_technician(self, technician_id: str, time_slot: str, booking_data: Dict[str, Any]) -> Dict[str, Any]:
        self.connections += 1
        await asyncio.sleep(random.uniform(0.015, 0.07))
        key = f"{technician_id}-{time_slot}"
        self.technician_bookings[key] = {
            'technician_id': technician_id,
            'time_slot': time_slot,
            **booking_data,
            'created_at': datetime.now().isoformat()
        }
        self.connections -= 1
        return {'success': True, 'booking_id': booking_data['booking_id']}

    def remove_booking(self, property_id: str, time_slot: str, booking_id: str) -> Dict[str, Any]:
        key = f"{property_id}-{time_slot}"
        booking = self.bookings.get(key)
        if booking and booking['booking_id'] == booking_id:
            del self.bookings[key]
            return {'success': True}
        return {'success': False, 'reason': 'Booking not found'}

    def release_technician(self, technician_id: str, time_slot: str, booking_id: str) -> Dict[str, Any]:
        key = f"{technician_id}-{time_slot}"
        booking = self.technician_bookings.get(key)
        if booking and booking['booking_id'] == booking_id:
            del self.technician_bookings[key]
            return {'success': True}
        return {'success': False, 'reason': 'Technician booking not found'}

    def get_booking(self, property_id: str, time_slot: str) -> Optional[Dict[str, Any]]:
        key = f"{property_id}-{time_slot}"
        return self.bookings.get(key)

    def get_technician_booking(self, technician_id: str, time_slot: str) -> Optional[Dict[str, Any]]:
        key = f"{technician_id}-{time_slot}"
        return self.technician_bookings.get(key)


class BookingSystem:
    def __init__(self):
        self.database = MockDatabase()
        self.active_bookings: Dict[str, Dict[str, Any]] = {}
        self.property_locks = defaultdict(asyncio.Lock)
        self.technician_locks = defaultdict(asyncio.Lock)

    def initialize(self):
        self.database.initialize()

    async def is_available(self, property_id: str, time_slot: str) -> bool:
        return await self.database.check_availability(property_id, time_slot)

    async def is_technician_available(self, technician_id: str, time_slot: str) -> bool:
        return await self.database.check_technician_availability(technician_id, time_slot)

    async def schedule_maintenance_request(self, property_id: str, time_slot: str, technician_id: str, booking_data: Dict[str, Any]) -> Dict[str, Any]:
        booking_id = f"booking-{int(time.time() * 1000)}-{random.randint(100000, 999999)}"
        property_key = f"{property_id}-{time_slot}"
        technician_key = f"{technician_id}-{time_slot}"

        # Consistent locking order to avoid deadlocks
        first_lock, second_lock = sorted(
            [self.property_locks[property_key], self.technician_locks[technician_key]],
            key=id
        )

        async with first_lock:
            async with second_lock:
                try:
                    # Check property availability
                    if not await self.is_available(property_id, time_slot):
                        return {
                            'success': False,
                            'reason': 'Property not available',
                            'booking_id': booking_id
                        }

                    # Check technician availability
                    if not await self.is_technician_available(technician_id, time_slot):
                        return {
                            'success': False,
                            'reason': 'Technician not available',
                            'booking_id': booking_id
                        }

                    # Simulate business logic processing
                    await asyncio.sleep(random.uniform(0.05, 0.15))
                    await asyncio.sleep(random.uniform(0.02, 0.08))  # payment processing
                    await asyncio.sleep(random.uniform(0.03, 0.09))  # technician validation

                    # Reserve technician
                    technician_result = await self.database.reserve_technician(technician_id, time_slot, {
                        'booking_id': booking_id,
                        'property_id': property_id,
                        'type': 'maintenance_request',
                        'cost': booking_data.get('cost', 200),
                        'description': booking_data.get('description', 'Maintenance request'),
                        'priority': booking_data.get('priority', 'medium')
                    })

                    if not technician_result['success']:
                        return {
                            'success': False,
                            'reason': 'Failed to reserve technician',
                            'booking_id': booking_id
                        }

                    # Create property booking
                    property_result = await self.database.create_booking(property_id, time_slot, technician_id, {
                        'booking_id': booking_id,
                        'type': 'maintenance_request',
                        'cost': booking_data.get('cost', 200),
                        'description': booking_data.get('description', 'Maintenance request'),
                        'priority': booking_data.get('priority', 'medium')
                    })

                    if property_result['success']:
                        self.active_bookings[booking_id] = {
                            'property_id': property_id,
                            'time_slot': time_slot,
                            'technician_id': technician_id,
                            **booking_data,
                            'status': 'confirmed',
                            'created_at': datetime.now().isoformat()
                        }
                        return {'success': True, 'booking_id': booking_id}
                    else:
                        self.database.release_technician(technician_id, time_slot, booking_id)
                        return {
                            'success': False,
                            'reason': property_result['reason'],
                            'booking_id': booking_id
                        }

                except Exception:
                    return {
                        'success': False,
                        'reason': 'Internal error',
                        'booking_id': booking_id
                    }

    def cancel_booking(self, booking_id: str) -> Dict[str, Any]:
        booking = self.active_bookings.get(booking_id)
        if not booking:
            return {'success': False, 'reason': 'Booking not found'}
        try:
            self.database.remove_booking(booking['property_id'], booking['time_slot'], booking_id)
            self.database.release_technician(booking['technician_id'], booking['time_slot'], booking_id)
            del self.active_bookings[booking_id]
            return {'success': True}
        except Exception:
            return {'success': False, 'reason': 'Cancellation failed'}

    def get_booking_status(self, booking_id: str) -> Optional[Dict[str, Any]]:
        return self.active_bookings.get(booking_id)

    def clear(self):
        self.database.clear()
        self.active_bookings.clear()
