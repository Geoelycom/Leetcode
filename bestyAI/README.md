# Property Management System Challenge

## Background

You've been brought in to help debug a critical issue with our property management system. Our platform allows prospective tenants to book exclusive maintenance appointments for rental properties, and we've been experiencing a serious problem that's causing operational headaches.

---

## The Problem

Our property managers have been reporting two problems:

- **Properties that should only allow one exclusive maintenance appointment per time slot are somehow getting multiple bookings**
- **Technicians are getting double-booked and showing up to conflicting appointments**

---

## Business Rules

Our system is designed with the following constraints:

- **One exclusive appointment per property per time slot** – each appointment should be focused and private
- **One appointment per technician per time slot** – technicians can only be in one place at a time
- **Property availability** – only available properties can be booked for maintenance
- **Technician availability** – only available technicians can be assigned to appointments
- **Appointment tracking** – all bookings must be properly recorded with both property and technician details
- **No conflicts** – no double-booking of either properties or technicians

---

## Current System Architecture

The system consists of:

- **BookingSystem** – Main business logic for coordinating property and technician bookings
- **MockDatabase** – Simulates database operations with realistic network delays for both properties and technicians
- **Two-phase booking process:** technician reservation followed by property booking
- **Appointment scheduling and validation logic**

---

## What We've Observed

The issue seems to be most prominent during high-traffic periods when multiple users are trying to book appointments for popular properties or with popular technicians at the same time. During off-peak hours with sequential bookings, the system works correctly and properly prevents conflicts.

Our logs show that somehow multiple booking requests are all returning "success" responses even when they should be rejected due to either the property time slot or technician already being taken.

---

## Your Task

1. **Choose your preferred language (JavaScript or Python)**
2. **Investigate the codebase** to understand how the booking system coordinates properties and technicians
3. **Run the test suite** to reproduce the issue we're experiencing in production
4. **Identify the root cause** of why our conflict prevention is being bypassed
5. **Implement a solution** that ensures our business rules are properly enforced for both resources
6. **Verify your fix** by ensuring all tests pass consistently

---

## Getting Started

### Choose Your Language

You can work in either JavaScript or Python – both have identical functionality and the same bug. Pick whichever you're more comfortable with.

#### Option A: JavaScript

```sh
cd property-js

# Install dependencies
npm install

# Run the test suite to see the current behavior
npm test
```

#### Option B: Python

```sh
cd property-python

# Install dependencies
pip install -r requirements.txt

# Run the test suite to see the current behavior
pytest test_booking_system.py -v -s
```

---

## Test Results You Should See

When you run the tests, you should see that basic functionality works fine:

- Single bookings work correctly
- Sequential bookings prevent conflicts
- Cancellation and rebooking works properly for both properties and technicians

However, there are several tests that consistently fail:

- Concurrent booking scenarios allow multiple successful bookings for the same property/time slot
- Concurrent booking scenarios allow multiple successful bookings for the same technician/time slot
- High-volume simultaneous requests create conflicts

Pay attention to the console output during the failing tests - it will show you exactly what's going wrong with messages like:

```
CONCURRENT PROPERTY BOOKING ISSUE DETECTED!
   Multiple bookings succeeded for the same property/time slot: 3
   Only 1 booking should succeed per property/time slot

CONCURRENT TECHNICIAN BOOKING ISSUE DETECTED!
   Multiple bookings succeeded for the same technician/time slot: 2
   Only 1 booking should succeed per technician/time slot
```

---

## Success Criteria

Your solution should:

- Prevent double-booking of properties under high concurrent load
- Prevent double-booking of technicians under high concurrent load
- Maintain all existing functionality
- Handle realistic network delays and timing variations
- Pass all tests consistently (run them multiple times to be sure)
- Follow good software engineering practices

---

## Time Limit

You have 12 minutes to complete this challenge. Good luck